// Basic HTML content parsing
use anyhow::Result;
use core::str;
use html_escape::decode_html_entities;
use std::io::{BufRead, stdin};

/// HTML content parser
struct Parser {
    /// DOM element stack (element ID, display)
    stack: Vec<(String, bool)>,
}

/// Main entry point
fn main() -> Result<()> {
    let mut content = Parser::new();
    content.parse(stdin().lock())?;
    Ok(())
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

impl Parser {
    /// Create a new HTML content parser
    fn new() -> Self {
        Parser { stack: Vec::new() }
    }

    /// Is content is currently displayed?
    fn is_content_displayed(&self) -> bool {
        !self.stack.iter().any(|(_, d)| !d)
    }

    /// Parse HTML
    fn parse<R>(&mut self, mut reader: R) -> Result<()>
    where
        R: BufRead,
    {
        let mut bytes = [0; 1];
        let mut buf = Vec::with_capacity(256);
        let mut quote = None;
        loop {
            if reader.read(&mut bytes)? == 0 {
                break;
            }
            let b = bytes[0];
            if quote.is_none() {
                if b'\\' == b {
                    quote = Some(b);
                    continue;
                } else if b'"' == b || b'\'' == b {
                    buf.push(b);
                    quote = Some(b);
                    continue;
                }
            }
            if let Some(q) = quote {
                if b'\\' == q {
                    buf.push(b);
                    quote = None;
                    continue;
                } else if q == b {
                    buf.push(b);
                    quote = None;
                    continue;
                }
            }
            if b'>' == b {
                if let Some((last, _)) = self.stack.last() {
                    if last.as_str() == "style" && !buf.starts_with(b"style") {
                        buf.push(b);
                        continue;
                    }
                }
                self.handle_element(&buf);
                buf.clear();
                continue;
            }
            if b'<' == b {
                self.handle_content(&buf);
                buf.clear();
                continue;
            }
            buf.push(b);
        }
        Ok(())
    }

    /// Handle element
    fn handle_element(&mut self, buf: &[u8]) {
        let Ok(text) = str::from_utf8(&buf) else {
            eprintln!("Invalid UTF-8: {buf:?}");
            return;
        };
        if text.ends_with('/')
            || NON_CLOSING.iter().any(|tag| text.starts_with(tag))
        {
            return;
        }
        let mut parts = text.trim().split_whitespace();
        let Some(elem) = parts.next() else {
            eprintln!("Empty element!");
            return;
        };
        if elem.starts_with('/') {
            if let Some((pe, _d)) = self.stack.pop() {
                if elem[1..] != pe {
                    eprintln!("tag: {elem} != {pe}");
                }
            } else {
                eprintln!("stack empty!");
            }
        } else {
            let displayed = !NON_DISPLAYED.contains(&elem);
            let hidden = parts.any(is_class_hidden);
            let display = displayed && !hidden;
            self.stack.push((elem.to_string(), display));
        }
    }

    /// Handle content
    fn handle_content(&self, buf: &[u8]) {
        if self.is_content_displayed() {
            let Ok(text) = str::from_utf8(&buf) else {
                eprintln!("Invalid UTF-8: {buf:?}");
                return;
            };
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
