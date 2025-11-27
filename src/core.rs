use std::marker::PhantomData;

const DEFAULT_BUFFER_CAPACITY: usize = 128;

pub trait CanAddAttributes {}

/// Tag has been opened and allows attributes to be added <tag...
pub struct Open;

impl CanAddAttributes for Open {}

/// Tag has closed and no longer allows attributes, only content <tag>...
pub struct Content;

/// Tag is self closing and closes immediately <tag ... />
pub struct Void;

impl CanAddAttributes for Void {}

pub trait HasAttributes {
    /// Add a custom attribute to the element
    fn attr(self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self;

    /// Add a boolean attribute to the element
    fn flag(self, k: impl AsRef<str>) -> Self;
}

pub trait CanAddChildren {}

pub trait CanAddText {}

pub struct Node<Tag, State = Open> {
    tag: &'static [u8],
    buf: Vec<u8>,
    _state: PhantomData<State>,
    _tag: PhantomData<Tag>,
}

impl<Tag> Node<Tag, Open> {
    pub fn new(tag: &'static str) -> Self {
        let buf = Vec::with_capacity(DEFAULT_BUFFER_CAPACITY);
        Self::with_buffer(tag, buf)
    }

    pub fn with_buffer(tag: &'static str, mut buf: Vec<u8>) -> Self {
        buf.push(b'<');
        buf.extend_from_slice(tag.as_bytes());

        Node {
            tag: tag.as_bytes(),
            buf,
            _state: PhantomData,
            _tag: PhantomData,
        }
    }

    pub fn close(mut self) -> Node<Tag, Content> {
        self.buf.extend_from_slice(b">");
        Node {
            tag: self.tag,
            buf: self.buf,
            _state: PhantomData,
            _tag: PhantomData,
        }
    }
}

impl<State, Tag> Node<Tag, State> {
    pub fn map<F>(self, fun: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        fun(self)
    }

    pub fn map_when<F>(self, condition: bool, fun: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if condition { fun(self) } else { self }
    }
}

impl<Tag, State> HasAttributes for Node<Tag, State>
where
    State: CanAddAttributes,
{
    fn attr(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        let k = k.as_ref();
        let v = v.as_ref();
        self.buf.reserve(k.len() + v.len() + 8);

        self.buf.push(b' ');
        write_normalized(&mut self.buf, k);
        self.buf.extend_from_slice(b"=\"");
        write_escaped(&mut self.buf, v);
        self.buf.push(b'"');
        self
    }

    fn flag(mut self, k: impl AsRef<str>) -> Self {
        let k = k.as_ref();
        self.buf.reserve(k.len() + 8);

        self.buf.push(b' ');
        write_normalized(&mut self.buf, k);
        self
    }
}

impl<Tag> Node<Tag, Content>
where
    Tag: CanAddChildren,
{
    pub fn child(mut self, child: impl Renderable) -> Node<Tag, Content> {
        child.render_into(&mut self.buf);
        self
    }

    pub fn children<It, Fn, T, R>(mut self, iter: It, mut fun: Fn) -> Self
    where
        It: IntoIterator<Item = T>,
        Fn: FnMut(T) -> R,
        R: Renderable,
    {
        for item in iter {
            let elem = fun(item);
            elem.render_into(&mut self.buf);
        }

        self
    }

    pub fn child_when<Fn, T>(mut self, condition: bool, f: Fn) -> Self
    where
        Fn: FnOnce() -> Node<T, Content>,
    {
        if condition {
            let child = f();
            child.render_into(&mut self.buf);
        }
        self
    }
}

impl<Tag> Node<Tag, Open>
where
    Tag: CanAddChildren,
{
    pub fn child(self, child: impl Renderable) -> Node<Tag, Content> {
        self.close().child(child)
    }

    pub fn children<It, Fn, T, R>(self, iter: It, fun: Fn) -> Node<Tag, Content>
    where
        It: IntoIterator<Item = T>,
        Fn: FnMut(T) -> R,
        R: Renderable,
    {
        self.close().children(iter, fun)
    }

    pub fn child_when<Fn, T>(self, condition: bool, f: Fn) -> Node<Tag, Content>
    where
        Fn: FnOnce() -> Node<T, Content>,
    {
        self.close().child_when(condition, f)
    }
}

impl<Tag> Node<Tag, Open>
where
    Tag: CanAddText,
{
    pub fn text(self, text: impl AsRef<str>) -> Node<Tag, Content> {
        self.close().text(text.as_ref())
    }

    pub fn raw(self, text: impl AsRef<str>) -> Node<Tag, Content> {
        self.close().raw(text.as_ref())
    }
}

impl<Tag> Node<Tag, Content>
where
    Tag: CanAddText,
{
    pub fn text(mut self, text: impl AsRef<str>) -> Self {
        write_escaped(&mut self.buf, text.as_ref());
        self
    }

    pub fn raw(mut self, text: impl AsRef<str>) -> Self {
        self.buf.extend_from_slice(text.as_ref().as_bytes());
        self
    }
}

impl<Tag> Node<Tag, Void> {
    pub fn new_self_closing(tag: &'static str) -> Self {
        let mut buf = Vec::with_capacity(DEFAULT_BUFFER_CAPACITY);
        buf.push(b'<');
        buf.extend_from_slice(tag.as_bytes());

        Node {
            tag: tag.as_bytes(),
            buf,
            _state: PhantomData,
            _tag: PhantomData,
        }
    }
}

pub trait Renderable {
    fn render_into(self, buf: &mut Vec<u8>);

    fn render(self) -> String;
}

impl<Tag> Renderable for Node<Tag, Open> {
    fn render_into(self, buf: &mut Vec<u8>) {
        self.close().render_into(buf);
    }

    fn render(self) -> String {
        self.close().render()
    }
}

impl<Tag> Renderable for Node<Tag, Content> {
    fn render_into(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.buf);

        // close tag
        buf.extend_from_slice(b"</");
        buf.extend_from_slice(self.tag);
        buf.push(b'>');
    }

    fn render(mut self) -> String {
        // close tag
        self.buf.extend_from_slice(b"</");
        self.buf.extend_from_slice(self.tag);
        self.buf.push(b'>');

        String::from_utf8(self.buf).expect("Internal Error: Invalid UTF-8")
    }
}

impl<Tag> Renderable for Node<Tag, Void> {
    fn render_into(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.buf);
        buf.extend_from_slice(b" />");
    }

    fn render(mut self) -> String {
        self.buf.extend_from_slice(b" />");

        String::from_utf8(self.buf).expect("Internal Error: Invalid UTF-8")
    }
}

impl<Tag> Into<String> for Node<Tag, Open> {
    fn into(self) -> String {
        self.render()
    }
}

impl<Tag> Into<String> for Node<Tag, Content> {
    fn into(self) -> String {
        self.render()
    }
}

impl<Tag> Into<String> for Node<Tag, Void> {
    fn into(self) -> String {
        self.render()
    }
}

#[inline(always)]
fn write_normalized(dest: &mut Vec<u8>, k: &str) {
    let bytes = k.as_bytes();

    if !bytes.iter().any(|&b| b == b'_' || (b >= b'A' && b <= b'Z')) {
        dest.extend_from_slice(bytes);
        return;
    }

    dest.reserve(bytes.len());

    for &b in bytes {
        dest.push(match b {
            b'A'..=b'Z' => b + 32,
            b'_' => b'-',
            _ => b,
        });
    }
}

#[inline(always)]
pub fn write_escaped(dest: &mut Vec<u8>, src: &str) {
    let bytes = src.as_bytes();
    let len = bytes.len();

    if !bytes
        .iter()
        .any(|&b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''))
    {
        dest.extend_from_slice(bytes);
        return;
    }

    dest.reserve(len / 10);

    let mut start = 0;
    for (i, &b) in bytes.iter().enumerate() {
        let entity = match b {
            b'&' => Some(&b"&amp;"[..]),
            b'<' => Some(&b"&lt;"[..]),
            b'>' => Some(&b"&gt;"[..]),
            b'"' => Some(&b"&quot;"[..]),
            b'\'' => Some(&b"&#39;"[..]),
            _ => None,
        };

        if let Some(ent) = entity {
            dest.extend_from_slice(&bytes[start..i]);
            dest.extend_from_slice(ent);
            start = i + 1;
        }
    }
    dest.extend_from_slice(&bytes[start..]);
}
