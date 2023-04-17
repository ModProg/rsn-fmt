//! Formatter for [`rsn`]
#![warn(clippy::pedantic, missing_docs)]
use std::fmt::{Display, Write};
use std::ops::Range;

use rsn::tokenizer::{self, Balanced, Token, TokenKind, Tokenizer};
use thiserror::Error;

/// Configuration for `rsnfmt`
pub mod config;
pub use config::Config;
mod utils;
#[allow(clippy::wildcard_imports)]
use utils::*;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
/// Error returned from [`format_str`]
pub enum Error {
    /// Error Originating from Tokenization
    #[error("tokenizer error: {_0:?}")]
    Tokenizer(#[from] tokenizer::Error),
    /// Missmatched delimiter e.g. `( ... ]` or `... }`
    #[error("missmatched delimiter at {_0:?}")]
    MissmatchedDelimiter(Range<usize>),
}

/// Unwrapping write, because we only write to [`String`]
// Tried to shadow `std::write` but ra doesn't like: https://github.com/rust-lang/rust-analyzer/issues/13683
macro_rules! w {
    ($($tt:tt)*) => {
        { write!($($tt)*).unwrap(); }
    };
}

struct Indent {
    level: usize,
    hard_tab: bool,
    width: usize,
}

impl Indent {
    fn inc(&mut self) {
        self.level += 1;
    }

    fn dec(&mut self) {
        self.level -= 1;
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hard_tab {
            write!(f, "{:1$\t}", "", self.level)
        } else {
            write!(f, "{:1$}", "", self.width * self.level)
        }
    }
}

/// # Errors
/// Errors on syntactically invalid rsn.
pub fn format_str(source: &str, config: &Config) -> Result<String> {
    let mut tokenizer = Tokenizer::full(source);
    let mut f = String::new();
    let mut indent = config.indent();
    let mut opened = Vec::new();
    let mut nled = false;
    let mut spaced = false;
    let nl = config.line_ending;
    while let Some(token) = tokenizer.next() {
        let Token { location, kind } = token?;
        match kind {
            TokenKind::Integer(_)
            | TokenKind::Float(_)
            | TokenKind::Bool(_)
            | TokenKind::Character(_)
            | TokenKind::Byte(_)
            | TokenKind::String(_) // TODO align escaped newline
            | TokenKind::Bytes(_)  // TODO align escaped newline
            | TokenKind::Identifier(_)
            | TokenKind::Comment(_) // TODO indent
            => {
                if nled {
                    w!(f, "{indent}{}", &source[location]);
                } else {
                    if spaced {
                        w!(f, " ");
                    }
                    w!(f, "{}", &source[location]);
                }
            }
            TokenKind::Colon => w!(f, ":"),
            TokenKind::Comma => w!(f, ",{nl}"),
            TokenKind::Open(delimiter) => {
                let tmp = tokenizer.clone();
                match format_single_line(source, &mut tokenizer, delimiter, config)? {
                    Some(single_line) if f.lines().last().unwrap_or_default().len() + single_line.len() < config.max_width =>  {
                        if spaced || delimiter == Balanced::Brace {
                            w!(f, " ");
                        }
                        w!(f, "{single_line}");
                    }
                    _ => {
                        indent.inc();
                        opened.push(delimiter);
                        if spaced || delimiter.is_brace() {
                            w!(f, " {}{nl}", delimiter.open());
                        } else {
                            w!(f, "{}{nl}", delimiter.open());
                        }
                        tokenizer = tmp;
                    }
                }
            }
            TokenKind::Close(delimiter) => {
                indent.dec();
                if nled {
                    w!(f, "{indent}{}", delimiter.close());
                } else {
                    w!(f, "{nl}{indent}{}", delimiter.close());
                }
                if opened.is_empty() || delimiter != opened.pop().expect("opened is not empty") {
                    return Err(Error::MissmatchedDelimiter(location));
                }
            }
            TokenKind::Whitespace => {
                let ws = &source[location.clone()];
                match config.preserve_empty_lines {
                    config::PreserveEmptyLines::One => {
                        if ws.chars().filter(|c|*c=='\n').count() > 1 {
                            if !nled {
                                w!(f, "{nl}");
                            }
                            w!(f, "{nl}");
                            nled = true;
                            spaced = false;
                        }
                    },
                    config::PreserveEmptyLines::All => for _ in 0..ws.chars().filter(|c|*c=='\n').count().saturating_sub(usize::from(nled)) {
                        w!(f, "{nl}");
                        nled = true;
                        spaced = false;
                    },
                    config::PreserveEmptyLines::None => {},
                }
            }
        }
        if kind != TokenKind::Whitespace {
            nled = matches!(kind, TokenKind::Comma | TokenKind::Open(_));
            spaced = kind == TokenKind::Colon;
        }
    }
    Ok(f)
}

fn format_single_line(
    source: &str,
    tokenizer: &mut Tokenizer<true>,
    delimiter: Balanced,
    config: &Config,
) -> Result<Option<String>> {
    let mut f = String::new();
    let mut opened = vec![delimiter];
    let mut spaced = delimiter == Balanced::Brace;
    let mut comma = false;
    let mut empty = true;
    w!(f, "{}", delimiter.open());
    for token in tokenizer {
        let Token { location, kind } = token?;
        if comma {
            match kind {
                TokenKind::Integer(_)
                | TokenKind::Float(_)
                | TokenKind::Bool(_)
                | TokenKind::Character(_)
                | TokenKind::Byte(_)
                | TokenKind::String(_)
                | TokenKind::Bytes(_)
                | TokenKind::Identifier(_)
                | TokenKind::Open(_) => {
                    w!(f, ",");
                    comma = false;
                }
                TokenKind::Close(_) => comma = false,
                _ => {}
            }
        }
        match kind {
            TokenKind::Byte(_) | TokenKind::String(_)
                if source[location.clone()].contains('\n') =>
            {
                return Ok(None);
            }
            TokenKind::Open(_) if opened.len() > config.max_inline_level => {
                return Ok(None);
            }
            TokenKind::Close(_) if !empty && opened.len() > config.max_inline_level => {
                return Ok(None);
            }
            TokenKind::Integer(_)
            | TokenKind::Float(_)
            | TokenKind::Bool(_)
            | TokenKind::Character(_)
            | TokenKind::Byte(_)
            | TokenKind::String(_)
            | TokenKind::Bytes(_)
            | TokenKind::Identifier(_) => {
                if spaced {
                    w!(f, " ");
                }
                w!(f, "{}", &source[location]);
            }
            TokenKind::Comment(_) => {
                todo!("comment")
            }
            TokenKind::Colon => {
                w!(f, ":");
            }
            TokenKind::Comma => comma = true,
            TokenKind::Open(delimiter) => {
                opened.push(delimiter);
                if spaced || delimiter == Balanced::Brace {
                    w!(f, " ");
                }
                w!(f, "{}", delimiter.open());
            }
            TokenKind::Close(delimiter) => {
                w!(f, "{}", delimiter.close());
                if opened.is_empty() || delimiter != opened.pop().expect("opened is not empty") {
                    return Err(Error::MissmatchedDelimiter(location));
                }
                if opened.is_empty() {
                    return Ok(Some(f));
                }
            }
            TokenKind::Whitespace => {
                if source[location.clone()]
                    .chars()
                    .filter(|c| *c == '\n')
                    .count()
                    > 1
                    && !config.preserve_empty_lines.is_none()
                {
                    return Ok(None);
                }
            }
        }
        if kind != TokenKind::Whitespace {
            spaced = matches!(
                kind,
                TokenKind::Colon | TokenKind::Comma | TokenKind::Open(Balanced::Brace)
            );
        }
        empty |= !(kind.is_value() || kind.is_comment() || kind.is_close());
    }
    Ok(Some(f))
}
