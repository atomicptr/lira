use std::{borrow::Cow, marker::PhantomData};

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
    fn attr(self, k: impl Into<Cow<'static, str>>, v: impl AsRef<str>) -> Self;

    /// Add a boolean attribute to the element
    fn flag(self, k: impl Into<Cow<'static, str>>) -> Self;
}

pub trait CanAddChildren {}

pub trait CanAddText {}

pub struct Node<State = Open, Tag = ()> {
    tag: &'static str,
    buffer: String,
    _state: PhantomData<State>,
    _tag: PhantomData<Tag>,
}

impl<Tag> Node<Open, Tag> {
    pub fn new(tag: &'static str) -> Self {
        Self::new_with_prefix(tag, "")
    }

    pub fn new_with_prefix(tag: &'static str, prefix: &'static str) -> Self {
        let mut buffer = String::with_capacity(128);
        buffer.push_str(prefix);
        buffer.push('<');
        buffer.push_str(tag);

        Node {
            tag,
            buffer,
            _state: PhantomData,
            _tag: PhantomData,
        }
    }

    fn finish_start_tag(mut self) -> Node<Content, Tag> {
        self.buffer.push('>');
        Node {
            tag: self.tag,
            buffer: self.buffer,
            _state: PhantomData,
            _tag: PhantomData,
        }
    }
}

pub fn normalize_attr_name(k: impl Into<Cow<'static, str>>) -> String {
    k.into().to_lowercase().replace("_", "-")
}

impl<T, Tag> HasAttributes for Node<T, Tag>
where
    T: CanAddAttributes,
{
    fn attr(mut self, k: impl Into<Cow<'static, str>>, v: impl AsRef<str>) -> Self {
        self.buffer.push(' ');
        self.buffer.push_str(normalize_attr_name(k).as_ref());
        self.buffer.push_str("=\"");
        self.buffer.push_str(&escape(v.as_ref()));
        self.buffer.push('"');
        self
    }

    fn flag(mut self, k: impl Into<Cow<'static, str>>) -> Self {
        self.buffer.push(' ');
        self.buffer.push_str(normalize_attr_name(k).as_ref());
        self
    }
}

impl<Tag> Node<Open, Tag>
where
    Tag: CanAddChildren,
{
    pub fn child(self, child: impl IntoNode) -> Node<Content, Tag> {
        self.finish_start_tag().child(child)
    }
}

impl<Tag> Node<Open, Tag>
where
    Tag: CanAddText,
{
    pub fn text(self, text: impl AsRef<str>) -> Node<Content, Tag> {
        self.finish_start_tag().text(text.as_ref())
    }

    pub fn raw(self, text: impl AsRef<str>) -> Node<Content, Tag> {
        self.finish_start_tag().raw(text.as_ref())
    }
}

impl<Tag> Node<Open, Tag> {
    pub fn render(self) -> String {
        self.finish_start_tag().render()
    }
}

impl<Tag> Node<Content, Tag> {
    pub fn render(mut self) -> String {
        self.buffer.push_str("</");
        self.buffer.push_str(self.tag);
        self.buffer.push('>');
        self.buffer
    }
}

impl<Tag> Node<Content, Tag>
where
    Tag: CanAddChildren,
{
    pub fn child(mut self, child: impl IntoNode) -> Node<Content, Tag> {
        child.render_into(&mut self.buffer);
        self
    }
}

impl<Tag> Node<Content, Tag>
where
    Tag: CanAddText,
{
    pub fn text(mut self, text: impl AsRef<str>) -> Self {
        self.buffer.push_str(&escape(text.as_ref()));
        self
    }

    pub fn raw(mut self, text: impl AsRef<str>) -> Self {
        self.buffer.push_str(text.as_ref());
        self
    }
}

impl<Tag> Node<Void, Tag> {
    pub fn new_self_closing(tag: &'static str) -> Self {
        let mut buffer = String::with_capacity(128);
        buffer.push('<');
        buffer.push_str(tag);

        Node {
            tag,
            buffer,
            _state: PhantomData,
            _tag: PhantomData,
        }
    }

    pub fn render(mut self) -> String {
        self.buffer.push_str(" />");
        self.buffer
    }
}

pub trait IntoNode {
    fn render_into(self, buf: &mut String);
}

impl<Tag> IntoNode for Node<Open, Tag> {
    fn render_into(self, buf: &mut String) {
        buf.push_str(&self.finish_start_tag().render());
    }
}

impl<Tag> IntoNode for Node<Content, Tag> {
    fn render_into(self, buf: &mut String) {
        buf.push_str(&self.render());
    }
}

impl<Tag> IntoNode for Node<Void, Tag> {
    fn render_into(self, buf: &mut String) {
        buf.push_str(&self.render());
    }
}

pub fn escape(input: &str) -> Cow<'_, str> {
    let mut need_escape = false;

    for c in input.chars() {
        if matches!(c, '&' | '<' | '>' | '"' | '\'') {
            need_escape = true;
            break;
        }
    }

    if !need_escape {
        return Cow::Borrowed(input);
    }

    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }

    Cow::Owned(out)
}
