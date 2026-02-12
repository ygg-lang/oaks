use oak_core::{Token, TokenType, UniversalTokenRole};

/// Javadoc token
pub type JavadocToken = Token<JavadocTokenType>;

/// Token types for the Javadoc lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum JavadocTokenType {
    /// The root of the document.
    Root,

    /// Spaces, tabs, and other whitespace characters.
    Whitespace,

    /// Line breaks.
    Newline,

    /// Start of a Javadoc comment (`/**`).
    CommentStart,

    /// End of a Javadoc comment (`*/`).
    CommentEnd,

    /// A Javadoc block tag (e.g., `@param`, `@return`).
    JavadocTag,

    /// A general tag.
    Tag,

    /// The `@param` tag.
    ParamTag,

    /// The `@return` tag.
    ReturnTag,

    /// The `@throws` tag.
    ThrowsTag,

    /// The `@see` tag.
    SeeTag,

    /// The `@since` tag.
    SinceTag,

    /// The `@author` tag.
    AuthorTag,

    /// The `@version` tag.
    VersionTag,

    /// The `@deprecated` tag.
    DeprecatedTag,

    /// The `@inheritDoc` tag.
    InheritDocTag,

    /// The `@summary` tag.
    SummaryTag,

    /// The `@code` tag.
    CodeTag,

    /// The `@literal` tag.
    LiteralTag,

    /// The `@value` tag.
    ValueTag,

    /// The `@exception` tag.
    ExceptionTag,

    /// The `@link` tag.
    LinkTag,

    /// The `@linkplain` tag.
    LinkPlainTag,

    /// An HTML start tag.
    HtmlTag,

    /// An HTML end tag.
    HtmlEndTag,

    /// The `<p>` HTML tag.
    HtmlPTag,

    /// The `<br>` HTML tag.
    HtmlBrTag,

    /// The `<code>` HTML tag.
    HtmlCodeTag,

    /// The `<pre>` HTML tag.
    HtmlPreTag,

    /// The `<ul>` HTML tag.
    HtmlUlTag,

    /// The `<ol>` HTML tag.
    HtmlOlTag,

    /// The `<li>` HTML tag.
    HtmlLiTag,

    /// The `<a>` HTML tag.
    HtmlATag,

    /// The `<img>` HTML tag.
    HtmlImgTag,

    /// The `<table>` HTML tag.
    HtmlTableTag,

    /// The `<tr>` HTML tag.
    HtmlTrTag,

    /// The `<td>` HTML tag.
    HtmlTdTag,

    /// The `<th>` HTML tag.
    HtmlThTag,

    /// The `<blockquote>` HTML tag.
    HtmlBlockquoteTag,

    /// The `<h1>` HTML tag.
    HtmlH1Tag,

    /// The `<h2>` HTML tag.
    HtmlH2Tag,

    /// The `<h3>` HTML tag.
    HtmlH3Tag,

    /// The `<h4>` HTML tag.
    HtmlH4Tag,

    /// The `<h5>` HTML tag.
    HtmlH5Tag,

    /// The `<h6>` HTML tag.
    HtmlH6Tag,

    /// The `<b>` HTML tag.
    HtmlBTag,

    /// The `<i>` HTML tag.
    HtmlITag,

    /// The `<em>` HTML tag.
    HtmlEmTag,

    /// The `<strong>` HTML tag.
    HtmlStrongTag,

    /// The `<span>` HTML tag.
    HtmlSpanTag,

    /// The `<div>` HTML tag.
    HtmlDivTag,

    /// The `<tt>` HTML tag.
    HtmlTtTag,

    /// The `<kbd>` HTML tag.
    HtmlKbdTag,

    /// The `<var>` HTML tag.
    HtmlVarTag,

    /// The `<samp>` HTML tag.
    HtmlSampTag,

    /// The `<sub>` HTML tag.
    HtmlSubTag,

    /// The `<sup>` HTML tag.
    HtmlSupTag,

    /// The `<small>` HTML tag.
    HtmlSmallTag,

    /// The `<big>` HTML tag.
    HtmlBigTag,

    /// The `<del>` HTML tag.
    HtmlDelTag,

    /// The `<ins>` HTML tag.
    HtmlInsTag,

    /// The `<cite>` HTML tag.
    HtmlCiteTag,

    /// The `<dfn>` HTML tag.
    HtmlDfnTag,

    /// The `<abbr>` HTML tag.
    HtmlAbbrTag,

    /// The `<acronym>` HTML tag.
    HtmlAcronymTag,

    /// The `<q>` HTML tag.
    HtmlQTag,

    /// Plain text.
    Text,

    /// An asterisk (`*`).
    Asterisk,

    /// An opening brace (`{`).
    LeftBrace,

    /// A closing brace (`}`).
    RightBrace,

    /// An opening parenthesis (`(`).
    LeftParen,

    /// A closing parenthesis (`)`).
    RightParen,

    /// An opening bracket (`[`).
    LeftBracket,

    /// A closing bracket (`]`).
    RightBracket,

    /// An at sign (`@`).
    At,

    /// A hash sign (`#`).
    Hash,

    /// End of file.
    Eof,

    /// An error token.
    Error,
}

impl TokenType for JavadocTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
