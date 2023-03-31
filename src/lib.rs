use std::fmt::{Display, Write};

use rsn::tokenizer::{Balanced, Token, TokenKind, Tokenizer};

const INDENT: usize = 4;
const NL: &str = "\n";

struct Indent(usize);

impl Indent {
    fn inc(&mut self) {
        self.0 += 1;
    }
    fn dec(&mut self) {
        self.0 -= 1;
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:1$}", "", INDENT * self.0)
    }
}

fn open(delimiter: Balanced) -> char {
    match delimiter {
        Balanced::Paren => '(',
        Balanced::Brace => '{',
        Balanced::Bracket => '[',
    }
}
fn close(delimiter: Balanced) -> char {
    match delimiter {
        Balanced::Paren => ')',
        Balanced::Brace => '}',
        Balanced::Bracket => ']',
    }
}

pub fn format_str(source: &str) -> String {
    let mut tokenizer = Tokenizer::full(source);
    let mut f = String::new();
    let mut indent = Indent(0);
    let mut opened = Vec::new();
    let mut nled = false;
    let mut spaced = false;
    while let Some(token) = tokenizer.next() {
        let Token { location, kind } = token.unwrap();
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
                    write!(f, "{indent}{}", &source[location]).unwrap();
                } else {
                    write!(f, "{}", &source[location]).unwrap();
                }
            }
            TokenKind::Colon => write!(f, ": ").unwrap(),
            TokenKind::Comma => write!(f, ",{NL}").unwrap(),
            TokenKind::Open(delimiter) => {
                // TODO try to align as single line first and only break if two wide
                indent.inc();
                opened.push(delimiter);
                if spaced {
                    write!(f, "{}{NL}", open(delimiter)).unwrap();
                } else {
                    write!(f, " {}{NL}", open(delimiter)).unwrap();
                }
            }
            TokenKind::Close(delimiter) => {
                indent.dec();
                if nled {
                    write!(f, "{indent}{}", close(delimiter)).unwrap();
                } else {
                    write!(f, "{NL}{indent}{}", close(delimiter)).unwrap();
                }
                assert_eq!(delimiter, opened.pop().unwrap(), "unmatched delimiters");
            }
            TokenKind::Whitespace => {
                // TODO
            }
        }
        if kind != TokenKind::Whitespace {
            nled = matches!(kind, TokenKind::Comma | TokenKind::Open(_));
            spaced = kind == TokenKind::Comma;
        }
    }
    f
}
