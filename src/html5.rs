use crate::core::{
    CanAddChildren, CanAddText, HasAttributes, Node, Open, Void, normalize_attr_name,
};

// attributes

/// Global HTML attributes that can be used on almost any element.
/// Based on: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
pub trait HasGlobalAttributes: HasAttributes + Sized {
    /// Provides a hint for generating a keyboard shortcut for the current element.
    /// Space-separated list of characters.
    fn accesskey(self, value: impl AsRef<str>) -> Self {
        let key = "accesskey";
        self.attr(key, value)
    }

    /// Space-separated list of CSS classes.
    fn class(self, value: impl AsRef<str>) -> Self {
        let key = "class";
        self.attr(key, value)
    }

    /// Unique identifier for the element, must be unique in the document.
    fn id(self, value: impl AsRef<str>) -> Self {
        let key = "id";
        self.attr(key, value)
    }

    /// Specifies the language of the element's content (BCP 47 tag).
    fn lang(self, value: impl AsRef<str>) -> Self {
        let key = "lang";
        self.attr(key, value)
    }

    /// Advisory information, usually shown as a tooltip.
    fn title(self, value: impl AsRef<str>) -> Self {
        let key = "title";
        self.attr(key, value)
    }

    /// Makes the element draggable via the Drag and Drop API.
    fn draggable(self) -> Self {
        self.flag("draggable")
    }

    /// Enables or disables spell checking on the element.
    fn spellcheck(self) -> Self {
        self.flag("spellcheck")
    }

    /// Automatically focus this element when the page or dialog loads.
    fn autofocus(self) -> Self {
        self.flag("autofocus")
    }

    /// Allows editing of the element's content by the user.
    fn contenteditable(self) -> Self {
        self.flag("contenteditable")
    }

    /// Provides an explicit tab order for the element.
    fn tabindex(self, value: i32) -> Self {
        self.attr("tabindex", value.to_string())
    }

    /// Hides the element from the page.
    fn hidden(self) -> Self {
        self.flag("hidden")
    }

    /// Sets automatic capitalization behavior for user input.
    fn autocapitalize(self, value: impl AsRef<str>) -> Self {
        self.attr("autocapitalize", value)
    }

    /// Forms a class of attributes, called custom data attributes, that allow proprietary information to be exchanged
    /// between the HTML and its DOM representation that may be used by scripts. All such custom data are available
    /// via the HTMLElement interface of the element the attribute is set on. The HTMLElement.dataset property gives
    /// access to them.
    fn data(self, key: &'static str, value: impl AsRef<str>) -> Self
    where
        Self: Sized + HasAttributes,
    {
        self.attr(format!("data-{}", normalize_attr_name(key)), value)
    }
}

impl<Tag> HasGlobalAttributes for Node<Open, Tag> {}
impl<Tag> HasGlobalAttributes for Node<Void, Tag> {}

pub trait HasHref: HasAttributes + Sized {
    /// The URL of the linked resource (absolute or relative).
    fn href(self, value: impl AsRef<str>) -> Self {
        self.attr("href", value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rel {
    /// Alternate representations of the current document.
    Alternate,

    /// Author of the current document or article.
    Author,

    /// Permalink for the nearest ancestor section.
    Bookmark,

    /// Preferred URL for the current document.
    Canonical,

    /// Tells the browser to preemptively perform DNS resolution for the target resource's origin.
    DnsPrefetch,

    /// The referenced document is not part of the same site as the current document.
    External,

    /// Link to context-sensitive help.
    Help,

    /// An icon representing the current document.
    Icon,

    /// Indicates that the main content of the current document is covered by the copyright license described l
    /// by the referenced document.
    License,

    /// Web app manifest.
    Manifest,

    /// Indicates that the current document is a part of a series and that the next document in the series is the
    /// referenced document.
    Next,

    /// Indicates that the current document's original author or publisher does not endorse the referenced document.
    NoFollow,

    /// Creates a top-level browsing context that is not an auxiliary browsing context if the hyperlink would create
    /// either of those, to begin with (i.e., has an appropriate target attribute value).
    NoOpener,

    /// No Referer header will be included. Additionally, has the same effect as noopener.
    NoReferrer,

    /// Creates an auxiliary browsing context if the hyperlink would otherwise create a top-level browsing context
    /// that is not an auxiliary browsing context (i.e., has "_blank" as target attribute value).
    Opener,

    /// Specifies that the user agent should preemptively fetch and cache the target resource as it is likely to
    /// be required for a followup navigation.
    Prefetch,

    /// Specifies that the user agent must preemptively fetch and cache the target resource for current navigation
    /// according to the potential destination given by the as attribute (and the priority associated with the
    /// corresponding destination).
    Preload,

    /// Indicates that the current document is a part of a series and that the previous document in the series
    /// is the referenced document.
    Prev,

    /// Gives a link to a resource that can be used to search through the current document and its related pages.
    Search,

    /// Imports a style sheet.
    Stylesheet,

    /// Gives a tag (identified by the given address) that applies to the current document.
    Tag,
}

pub trait HasRel: HasAttributes + Sized {
    fn rel(self, rel: Rel) -> Self {
        let value = match rel {
            Rel::Alternate => "alternate",
            Rel::Author => "author",
            Rel::Bookmark => "bookmark",
            Rel::Canonical => "canonical",
            Rel::DnsPrefetch => "dns-prefetch",
            Rel::External => "external",
            Rel::Help => "help",
            Rel::Icon => "icon",
            Rel::License => "license",
            Rel::Manifest => "manifest",
            Rel::Next => "next",
            Rel::NoFollow => "nofollow",
            Rel::NoOpener => "noopener",
            Rel::NoReferrer => "noreferrer",
            Rel::Opener => "opener",
            Rel::Prefetch => "prefetch",
            Rel::Preload => "preload",
            Rel::Prev => "prev",
            Rel::Search => "search",
            Rel::Stylesheet => "stylesheet",
            Rel::Tag => "tag",
        };
        self.attr("rel", value)
    }
}

pub trait HasSrc: HasAttributes + Sized {
    /// URL of the embeddable content (image, script, iframe, etc.).
    fn src(self, value: impl AsRef<str>) -> Self {
        self.attr("src", value)
    }
}

pub enum InputType {
    Text,
    Password,
    Checkbox,
    Radio,
    File,
    Submit,
    Reset,
    Button,
    Hidden,
    Email,
    Number,
    Date,
}

pub trait HasInputType: HasAttributes + Sized {
    fn input_type(self, value: impl Into<InputType>) -> Self {
        let value = match value.into() {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Checkbox => "checkbox",
            InputType::Radio => "radio",
            InputType::File => "file",
            InputType::Submit => "submit",
            InputType::Reset => "reset",
            InputType::Button => "button",
            InputType::Hidden => "hidden",
            InputType::Email => "email",
            InputType::Number => "number",
            InputType::Date => "date",
        };
        self.attr("type", value)
    }
}

pub enum Target {
    Self_,
    Blank,
    Parent,
    Top,
}

pub trait HasTarget: HasAttributes + Sized {
    fn target(self, target: Target) -> Self {
        let value = match target {
            Target::Self_ => "_self",
            Target::Blank => "_blank",
            Target::Parent => "_parent",
            Target::Top => "_top",
        };
        self.attr("target", value)
    }
}

pub enum FormMethod {
    Get,
    Post,
    Dialog,
}

// HTML elements

// <html> element

// <html>
pub struct Html;

/// Root element of an HTML document. All other elements must be descendants of this.
pub fn html() -> Node<Open, Html> {
    Node::new_with_prefix("html", "<!DOCTYPE html>")
}

impl CanAddChildren for Html {}

// <head>
pub struct Head;

/// Contains metadata about the document (title, links, meta, etc.).
pub fn head() -> Node<Open, Head> {
    Node::new("head")
}

impl CanAddChildren for Head {}

// <link> — void element
pub struct Link;

/// Specifies relationships between the current document and external resources (e.g. CSS, icons).
pub fn link() -> Node<Void, Link> {
    Node::new_self_closing("link")
}

impl HasHref for Node<Void, Link> {}
impl HasRel for Node<Void, Link> {}

impl Node<Void, Link> {
    /// Specifies a Subresource Integrity value that allows browsers to verify what they fetch.
    pub fn integrity(self, value: impl AsRef<str>) -> Self {
        self.attr("integrity", value)
    }
}

// <meta> — void element
pub struct Meta;

/// Represents metadata not covered by other elements.
pub fn meta() -> Node<Void, Meta> {
    Node::new_self_closing("meta")
}

impl Node<Void, Meta> {
    pub fn charset(self, value: impl AsRef<str>) -> Self {
        self.attr("charset", value)
    }
    pub fn content(self, value: impl AsRef<str>) -> Self {
        self.attr("content", value)
    }
    pub fn http_equiv(self, value: impl AsRef<str>) -> Self {
        self.attr("http-equiv", value)
    }
    pub fn media(self, value: impl AsRef<str>) -> Self {
        self.attr("media", value)
    }
    pub fn name(self, value: impl AsRef<str>) -> Self {
        self.attr("name", value)
    }
}

// <style>
pub struct Style;

/// Contains CSS styling rules for the document.
pub fn style() -> Node<Open, Style> {
    Node::new("style")
}

impl CanAddText for Style {}
impl CanAddChildren for Style {}

// <title>
pub struct Title;

/// Defines the document title shown in browser tab/title bar.
pub fn title() -> Node<Open, Title> {
    Node::new("title")
}

impl CanAddText for Title {}

// <body>
pub struct Body;

/// Represents the main content of the HTML document.
pub fn body() -> Node<Open, Body> {
    Node::new("body")
}

impl CanAddChildren for Body {}

// <div>
pub struct Div;

/// Generic block-level container for grouping content.
pub fn div() -> Node<Open, Div> {
    Node::new("div")
}

impl CanAddChildren for Div {}
impl CanAddText for Div {}

// <span>
pub struct Span;

/// Generic inline container for phrasing content.
pub fn span() -> Node<Open, Span> {
    Node::new("span")
}

impl CanAddChildren for Span {}
impl CanAddText for Span {}

// <h1>
pub struct H1;

pub fn h1() -> Node<Open, H1> {
    Node::new("h1")
}

impl CanAddChildren for H1 {}
impl CanAddText for H1 {}

// <h2>
pub struct H2;

pub fn h2() -> Node<Open, H2> {
    Node::new("h2")
}

impl CanAddChildren for H2 {}
impl CanAddText for H2 {}

// <h3>
pub struct H3;

pub fn h3() -> Node<Open, H3> {
    Node::new("h3")
}

impl CanAddChildren for H3 {}
impl CanAddText for H3 {}

// <h4>
pub struct H4;

pub fn h4() -> Node<Open, H4> {
    Node::new("h4")
}

impl CanAddChildren for H4 {}
impl CanAddText for H4 {}

// <h5>
pub struct H5;

pub fn h5() -> Node<Open, H5> {
    Node::new("h5")
}

impl CanAddChildren for H5 {}
impl CanAddText for H5 {}

// <h6>
pub struct H6;

pub fn h6() -> Node<Open, H6> {
    Node::new("h6")
}

impl CanAddChildren for H6 {}
impl CanAddText for H6 {}

// <p>
pub struct Paragraph;

/// Represents a paragraph of text.
pub fn p() -> Node<Open, Paragraph> {
    Node::new("p")
}

impl CanAddChildren for Paragraph {}
impl CanAddText for Paragraph {}

// <img>
pub struct Img;

/// Embeds an image into the document.
pub fn img() -> Node<Void, Img> {
    Node::new_self_closing("img")
}

impl HasSrc for Node<Void, Img> {}

impl Node<Void, Img> {
    /// Text description of the image, shown if image fails to load.
    pub fn alt(self, value: impl AsRef<str>) -> Self {
        self.attr("alt", value)
    }

    /// Intrinsic width of the image in pixels.
    pub fn width(self, value: i32) -> Self {
        self.attr("width", value.to_string())
    }

    /// Intrinsic height of the image in pixels.
    pub fn height(self, value: i32) -> Self {
        self.attr("height", value.to_string())
    }
}

// <form>
pub struct Form;

/// Represents a form, used to collect user input.
pub fn form() -> Node<Open, Form> {
    Node::new("form")
}

impl CanAddChildren for Form {}

impl Node<Open, Form> {
    /// URL to which the form data is submitted.
    pub fn action(self, value: impl AsRef<str>) -> Self {
        self.attr("action", value)
    }

    /// HTTP method to submit the form (Get, Post, Dialog).
    pub fn method(self, method: FormMethod) -> Self {
        self.attr(
            "method",
            match method {
                FormMethod::Get => "GET",
                FormMethod::Post => "POST",
                FormMethod::Dialog => "dialog",
            },
        )
    }

    /// Name of the character encodings the server accepts.
    pub fn accept_charset(self, value: impl AsRef<str>) -> Self {
        self.attr("accept-charset", value)
    }

    /// Toggles browser autocomplete for the form fields.
    pub fn autocomplete(self, value: bool) -> Self {
        self.attr("autocomplete", if value { "on" } else { "off" })
    }
}

// <input>
pub struct Input;

/// Represents an input field where users can enter data.
pub fn input() -> Node<Void, Input> {
    Node::new_self_closing("input")
}

impl HasInputType for Node<Void, Input> {}

impl Node<Void, Input> {
    /// Name of the input control.
    pub fn name(self, value: impl AsRef<str>) -> Self {
        self.attr("name", value)
    }

    /// Default value of the input.
    pub fn value(self, value: impl AsRef<str>) -> Self {
        self.attr("value", value)
    }

    /// Indicates that the input should be checked by default (for checkboxes/radios).
    pub fn checked(self) -> Self {
        self.flag("checked")
    }

    /// Specifies accepted file types for file input.
    pub fn accept(self, value: impl AsRef<str>) -> Self {
        self.attr("accept", value)
    }

    /// Enables capturing a new file via camera or microphone.
    pub fn capture(self) -> Self {
        self.flag("capture")
    }

    /// Enables browser autocomplete for the input.
    pub fn autocomplete(self, value: bool) -> Self {
        self.attr("autocomplete", if value { "on" } else { "off" })
    }
}

// <button>
pub struct Button;

/// Represents a clickable button.
pub fn button() -> Node<Open, Button> {
    Node::new("button")
}

impl CanAddChildren for Button {}
impl CanAddText for Button {}

impl Node<Open, Button> {
    /// Type of button: "button", "submit", or "reset".
    pub fn type_(self, value: impl AsRef<str>) -> Self {
        self.attr("type", value)
    }

    /// Name of the button.
    pub fn name(self, value: impl AsRef<str>) -> Self {
        self.attr("name", value)
    }

    /// Default value of the button.
    pub fn value(self, value: impl AsRef<str>) -> Self {
        self.attr("value", value)
    }

    /// Disables the button.
    pub fn disabled(self) -> Self {
        self.flag("disabled")
    }
}

// <textarea>
pub struct Textarea;

/// Represents a multi-line text input.
pub fn textarea() -> Node<Open, Textarea> {
    Node::new("textarea")
}

impl CanAddText for Textarea {}

impl Node<Open, Textarea> {
    /// Name of the textarea.
    pub fn name(self, value: impl AsRef<str>) -> Self {
        self.attr("name", value)
    }

    /// Default value inside the textarea.
    pub fn value(self, value: impl AsRef<str>) -> Self {
        self.attr("value", value)
    }

    /// Number of visible rows.
    pub fn rows(self, value: i32) -> Self {
        self.attr("rows", value.to_string())
    }

    /// Number of visible columns.
    pub fn cols(self, value: i32) -> Self {
        self.attr("cols", value.to_string())
    }

    /// Disables the textarea.
    pub fn disabled(self) -> Self {
        self.flag("disabled")
    }

    /// Enables browser autocomplete.
    pub fn autocomplete(self, value: bool) -> Self {
        self.attr("autocomplete", if value { "on" } else { "off" })
    }
}

// <select>
pub struct Select;

/// Represents a drop-down list of options.
pub fn select() -> Node<Open, Select> {
    Node::new("select")
}

impl CanAddChildren for Select {}

impl Node<Open, Select> {
    /// Name of the select element.
    pub fn name(self, value: impl AsRef<str>) -> Self {
        self.attr("name", value)
    }

    /// Disables the select element.
    pub fn disabled(self) -> Self {
        self.flag("disabled")
    }

    /// Enables multiple selection.
    pub fn multiple(self) -> Self {
        self.flag("multiple")
    }

    /// Enables browser autocomplete.
    pub fn autocomplete(self, value: bool) -> Self {
        self.attr("autocomplete", if value { "on" } else { "off" })
    }
}

// <option>
pub struct OptionElement;

/// Represents a single option inside a <select>.
pub fn option() -> Node<Open, OptionElement> {
    Node::new("option")
}

impl CanAddText for OptionElement {}

impl Node<Open, OptionElement> {
    /// Value of the option when submitted.
    pub fn value(self, value: impl AsRef<str>) -> Self {
        self.attr("value", value)
    }

    /// Marks the option as selected.
    pub fn selected(self) -> Self {
        self.flag("selected")
    }

    /// Disables the option.
    pub fn disabled(self) -> Self {
        self.flag("disabled")
    }
}

// <header>
pub struct Header;

/// Represents introductory content or a set of navigational links.
pub fn header() -> Node<Open, Header> {
    Node::new("header")
}

impl CanAddChildren for Header {}
impl CanAddText for Header {}

// <footer>
pub struct Footer;

/// Represents footer content for its nearest sectioning content.
pub fn footer() -> Node<Open, Footer> {
    Node::new("footer")
}

impl CanAddChildren for Footer {}
impl CanAddText for Footer {}

// <nav>
pub struct Nav;

/// Represents a section of navigation links.
pub fn nav() -> Node<Open, Nav> {
    Node::new("nav")
}

impl CanAddChildren for Nav {}
impl CanAddText for Nav {}

// <section>
pub struct Section;

/// Represents a generic section of content.
pub fn section() -> Node<Open, Section> {
    Node::new("section")
}

impl CanAddChildren for Section {}
impl CanAddText for Section {}

// <article>
pub struct Article;

/// Represents a self-contained composition, such as a blog post or news article.
pub fn article() -> Node<Open, Article> {
    Node::new("article")
}

impl CanAddChildren for Article {}
impl CanAddText for Article {}

// <aside>
pub struct Aside;

/// Represents content indirectly related to the main content (sidebar).
pub fn aside() -> Node<Open, Aside> {
    Node::new("aside")
}

impl CanAddChildren for Aside {}
impl CanAddText for Aside {}

// <main>
pub struct Main;

/// Represents the dominant content of the <body> of a document.
pub fn main() -> Node<Open, Main> {
    Node::new("main")
}

impl CanAddChildren for Main {}
impl CanAddText for Main {}

// <code>
pub struct Code;

/// Represents a fragment of computer code.
pub fn code() -> Node<Open, Code> {
    Node::new("code")
}

impl CanAddChildren for Code {}
impl CanAddText for Code {}

// <pre>
pub struct Pre;

/// Represents preformatted text.
pub fn pre() -> Node<Open, Pre> {
    Node::new("pre")
}

impl CanAddChildren for Pre {}
impl CanAddText for Pre {}

// <b>
pub struct B;

/// Represents text with bold importance.
pub fn b() -> Node<Open, B> {
    Node::new("b")
}

impl CanAddChildren for B {}
impl CanAddText for B {}

// <i>
pub struct I;

/// Represents text in italics, usually for emphasis or stylistic purposes.
pub fn i() -> Node<Open, I> {
    Node::new("i")
}

impl CanAddChildren for I {}
impl CanAddText for I {}

// <u>
pub struct U;

/// Represents text that should be stylistically underlined.
pub fn u() -> Node<Open, U> {
    Node::new("u")
}

impl CanAddChildren for U {}
impl CanAddText for U {}

// <strong>
pub struct Strong;

/// Represents text with strong importance.
pub fn strong() -> Node<Open, Strong> {
    Node::new("strong")
}

impl CanAddChildren for Strong {}
impl CanAddText for Strong {}

// <small>
pub struct Small;

/// Represents smaller print text.
pub fn small() -> Node<Open, Small> {
    Node::new("small")
}

impl CanAddChildren for Small {}
impl CanAddText for Small {}

// <label>
pub struct Label;

/// Represents a label for a form control.
pub fn label() -> Node<Open, Label> {
    Node::new("label")
}

impl CanAddChildren for Label {}
impl CanAddText for Label {}

impl Node<Open, Label> {
    /// Associates the label with a form control by its ID.
    pub fn for_(self, value: impl AsRef<str>) -> Self {
        self.attr("for", value)
    }
}

// <details>
pub struct Details;

/// Represents a disclosure widget that users can open or close.
pub fn details() -> Node<Open, Details> {
    Node::new("details")
}

impl CanAddChildren for Details {}
impl CanAddText for Details {}

impl Node<Open, Details> {
    /// Indicates whether the details should be open by default.
    pub fn open(self) -> Self {
        self.flag("open")
    }
}

// <summary>
pub struct Summary;

/// Represents a summary, visible in a <details> element.
pub fn summary() -> Node<Open, Summary> {
    Node::new("summary")
}

impl CanAddChildren for Summary {}
impl CanAddText for Summary {}

// <dialog>
pub struct Dialog;

/// Represents a dialog box or interactive window.
pub fn dialog() -> Node<Open, Dialog> {
    Node::new("dialog")
}

impl CanAddChildren for Dialog {}
impl CanAddText for Dialog {}

impl Node<Open, Dialog> {
    /// Indicates that the dialog is open.
    pub fn open(self) -> Self {
        self.flag("open")
    }
}

// <hr>
pub struct Hr;

/// Represents a thematic break (horizontal rule).
pub fn hr() -> Node<Void, Hr> {
    Node::new_self_closing("hr")
}

// <base>
pub struct Base;

/// Specifies the base URL and target for relative URLs.
pub fn base() -> Node<Void, Base> {
    Node::new_self_closing("base")
}

impl HasHref for Node<Void, Base> {}
impl HasTarget for Node<Void, Base> {}

// <script>
pub struct Script;

pub fn script() -> Node<Open, Script> {
    Node::new("script")
}

impl CanAddText for Script {}
impl HasSrc for Node<Open, Script> {}

impl Node<Open, Script> {
    /// Executes the script asynchronously.
    pub fn async_(self) -> Self {
        self.flag("async")
    }

    /// Indicates that the script should be executed after the page has been parsed.
    pub fn defer(self) -> Self {
        self.flag("defer")
    }

    /// Specifies a Subresource Integrity value that allows browsers to verify what they fetch.
    pub fn integrity(self, value: impl AsRef<str>) -> Self {
        self.attr("integrity", value)
    }

    /// Type of script: "importmap", "module", ...
    pub fn type_(self, value: impl AsRef<str>) -> Self {
        self.attr("type", value)
    }
}
