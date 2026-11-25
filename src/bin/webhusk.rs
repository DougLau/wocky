// Basic HTML husking (extracting text)
use anyhow::Result;
use core::str;
use html_escape::decode_html_entities;
use std::io::{BufRead, IsTerminal, stdin};
use wocky::parse::CharSplitter;

/// Parsing context
#[derive(Clone, Copy, Debug, PartialEq)]
enum Context {
    /// HTML parsing context
    Html,
    /// Backslash escape character in HTML context
    HtmlBackslash,
    /// Tag context (within '<' and '>')
    Tag,
    /// Backslash escape character in tag context
    TagBackslash,
    /// Single-quoted string in tag context
    SingleQuote,
    /// Double-quoted string in tag context
    DoubleQuote,
    /// CSS parsing context
    Css,
}

/// HTML content husker
struct HtmlHusker {
    /// DOM element stack (element ID, display)
    stack: Vec<(String, bool)>,
}

/// Non-closing entities (also comments)
const NON_CLOSING: &[&str] =
    &["!--", "br", "wbr", "img", "input", "link", "meta", "source"];

/// Non-displayed entity names
const NON_DISPLAYED: &[&str] = &[
    "annotation",
    "figure",
    "footer",
    "form",
    "header",
    "label",
    "nav",
    "script",
    "semantics",
    "style",
];

impl Context {
    /// Should a char be included?
    fn include(self, c: char) -> bool {
        use Context::*;
        match (self, c) {
            (SingleQuote | DoubleQuote, '<' | '>') => true,
            (_, '<' | '>') => false,
            (Css, _) => false,
            _ => true,
        }
    }

    /// Handle the next char
    fn handle_char(&mut self, c: char) {
        use Context::*;
        match (*self, c) {
            (Html, '\\') => *self = HtmlBackslash,
            (Html, '<') => *self = Tag,
            (HtmlBackslash, _c) => *self = Html,
            (Tag, '\\') => *self = TagBackslash,
            (Tag, '\'') => *self = SingleQuote,
            (Tag, '"') => *self = DoubleQuote,
            (Tag, '>') => *self = Html,
            (TagBackslash, _c) => *self = Tag,
            (SingleQuote, '\'') => *self = Tag,
            (DoubleQuote, '"') => *self = Tag,
            (Css, '<') => *self = Tag,
            _ => (),
        }
    }
}

impl HtmlHusker {
    /// Create a new HTML content husker
    fn new() -> Self {
        HtmlHusker { stack: Vec::new() }
    }

    /// Is content is currently displayed?
    fn is_content_displayed(&self) -> bool {
        !self.stack.iter().any(|(_, d)| !d)
    }

    /// Parse HTML
    fn parse<R>(&mut self, splitter: CharSplitter<R>) -> Result<()>
    where
        R: BufRead,
    {
        let mut text = String::with_capacity(256);
        let mut context = Context::Html;
        for c in splitter {
            let c = c?;
            let prev = context;
            context.handle_char(c);
            if context.include(c) {
                text.push(c);
            }
            if let (Context::Tag, Context::Html) = (prev, context) {
                self.handle_element(&text);
                if let Some((last, _)) = self.stack.last()
                    && last.as_str() == "style"
                {
                    context = Context::Css;
                }
                text.clear();
            }
            if let (Context::Html, Context::Tag) = (prev, context) {
                self.handle_content(&text);
                text.clear();
            }
        }
        Ok(())
    }

    /// Handle element
    fn handle_element(&mut self, text: &str) {
        if text.ends_with('/')
            || NON_CLOSING.iter().any(|tag| text.starts_with(tag))
        {
            return;
        }
        let mut parts = text.split_whitespace();
        let Some(tag) = parts.next() else {
            eprintln!("Empty element!");
            return;
        };
        if let Some(elem) = tag.strip_prefix('/') {
            while let Some((pe, _d)) = self.stack.pop() {
                if elem == pe {
                    break;
                }
            }
        } else {
            let displayed = !NON_DISPLAYED.contains(&tag);
            let hidden = parts.any(is_class_hidden);
            let display = displayed && !hidden;
            self.stack.push((tag.to_string(), display));
        }
    }

    /// Handle content
    fn handle_content(&self, text: &str) {
        if self.is_content_displayed() {
            let text = text.trim();
            if !text.is_empty() {
                println!("{}", decode_html_entities(text));
            }
        }
    }
}

/// Check if an HTML entity is hidden, based on class name (Wikipedia)
fn is_class_hidden(part: &str) -> bool {
    part.starts_with("class") && {
        part.contains("catlinks")
            || part.contains("sidebar-list")
            || part.contains("infobox")
            || part.contains("vector-menu")
            || part.contains("references")
            || part.contains("printfooter")
            || part.contains("mw-reflink-text")
            || part.contains("mw-editsection")
    }
}

/// Main entry point
fn main() -> Result<()> {
    let stdin = stdin();
    if stdin.is_terminal() {
        use yansi::Paint;
        eprintln!("{0} stdin must be redirected {0}", "!!!".bright_yellow());
        return Ok(());
    }
    let mut husker = HtmlHusker::new();
    husker.parse(CharSplitter::new(stdin.lock()))?;
    Ok(())
}
