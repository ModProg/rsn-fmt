use extend::ext;
use rsn::tokenizer::{Balanced, TokenKind};

#[ext]
pub(crate) impl Balanced {
    fn open(self) -> char {
        match self {
            Balanced::Paren => '(',
            Balanced::Brace => '{',
            Balanced::Bracket => '[',
        }
    }

    fn close(self) -> char {
        match self {
            Balanced::Paren => ')',
            Balanced::Brace => '}',
            Balanced::Bracket => ']',
        }
    }

    fn is_brace(self) -> bool {
        matches!(self, Self::Brace)
    }
}

#[ext]
pub(crate) impl TokenKind<'_> {
    fn is_value(&self) -> bool {
        matches!(
            self,
            TokenKind::Integer(_)
                | TokenKind::Float(_)
                | TokenKind::Bool(_)
                | TokenKind::Character(_)
                | TokenKind::Byte(_)
                | TokenKind::String(_)
                | TokenKind::Bytes(_)
                | TokenKind::Identifier(_)
        )
    }

    fn is_comment(&self) -> bool {
        matches!(self, TokenKind::Comment(_))
    }

    fn is_close(&self) -> bool {
        matches!(self, TokenKind::Close(_))
    }

    fn is_open(&self) -> bool {
        matches!(self, TokenKind::Open(_))
    }

    fn is_white_space(&self) -> bool {
        matches!(self, TokenKind::Whitespace(_))
    }
}
