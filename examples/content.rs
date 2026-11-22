// Basic HTML content parsing
use anyhow::Result;
use core::str;
use html_escape::decode_html_entities;
use std::io::{BufRead, stdin};

/// HTML content parser
struct Content {
    /// read buffer
    buf: Vec<u8>,
    /// DOM element stack (element ID, display)
    stack: Vec<(String, bool)>,
}

/// Main entry point
fn main() -> Result<()> {
    let mut content = Content::new();
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

impl Content {
    /// Create a new HTML content parser
    fn new() -> Self {
        Content {
            buf: Vec::with_capacity(4096),
            stack: Vec::new(),
        }
    }

    /// Is content is currently displayed?
    fn is_content_displayed(&self) -> bool {
        !self.stack.iter().any(|(_, d)| !d)
    }

    /// Handle content
    fn handle_content(&self) {
        if self.is_content_displayed() {
            if let Ok(text) = str::from_utf8(&self.buf) {
                let text = text.trim();
                if !text.is_empty() {
                    println!("{}", decode_html_entities(text));
                }
            }
        }
    }

    /// Parse HTML from a reader
    fn parse<R>(&mut self, mut reader: R) -> Result<()>
    where
        R: BufRead,
    {
        loop {
            self.buf.clear();
            if reader.read_until(b'<', &mut self.buf)? == 0 {
                break;
            }
            if let Some(b'<') = self.buf.pop() {
                self.handle_content();
                self.buf.clear();
                // FIXME: '>' can also be in quoted strings
                if reader.read_until(b'>', &mut self.buf)? == 0 {
                    break;
                }
                if let Some(b'>') = self.buf.pop() {
                    if let Ok(text) = str::from_utf8(&self.buf) {
                        let mut parts = text.trim().split_whitespace();
                        if let Some(elem) = parts.next() {
                            if elem.ends_with('/')
                                || NON_CLOSING
                                    .iter()
                                    .any(|tag| elem.starts_with(tag))
                            {
                                continue;
                            }
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
                    }
                }
            }
        }
        Ok(())
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
