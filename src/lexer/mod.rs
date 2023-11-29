use logos::{Lexer, Logos};

use crate::common::CoolDoc;

fn newline_callback(lex: &mut Lexer<TokenKind>) -> () {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

#[derive(Logos, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[logos(extras = (usize, usize))]
enum TokenKind {
    #[token("\n", newline_callback)]
    NewLine,
    #[regex(r"\s+")]
    Space,

    #[token("class", ignore(case))]
    Class,

    #[token("inherits", ignore(case))]
    Inherits,

    #[token("if", ignore(case))]
    If,

    #[token("then", ignore(case))]
    Then,

    #[token("else", ignore(case))]
    Else,

    #[token("fi", ignore(case))]
    Fi,

    #[token("while", ignore(case))]
    While,

    #[token("loop", ignore(case))]
    Loop,

    #[token("pool", ignore(case))]
    Pool,

    #[token("let", ignore(case))]
    Let,

    #[token("in", ignore(case))]
    In,

    #[token("case", ignore(case))]
    Case,

    #[token("of", ignore(case))]
    Of,

    #[token("esac", ignore(case))]
    Esac,

    #[token("new", ignore(case))]
    New,

    #[token("isvoid", ignore(case))]
    Isvoid,

    #[regex("[a-z][_a-zA-Z0-9]*")]
    ID,

    #[regex("[A-Z][_a-zA-Z0-9]*")]
    TYPE,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("not", ignore(case))]
    Not,

    #[token("=")]
    Equal,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token("<-")]
    Assign,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("[")]
    OpenSquare,

    #[token("]")]
    CloseSquare,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[regex(r"[0-9]+")]
    Integer,

    #[regex(r#""[^"]*""#)]
    String,

    #[token("true")]
    True,

    #[token("false")]
    False,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Token {
    kind: TokenKind,
    line: u32,
    column: u32,
    slice: &'static str,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let Token {
            line,
            column,
            kind,
            slice,
        } = self;
        format!("{line}:{column}:{kind:?} {slice}")
    }
}

impl CoolDoc {
    pub fn lex(self) -> Lexer<'static, TokenKind> {
        TokenKind::lexer(self.text())
    }

    pub fn tokenize(self) -> impl Iterator<Item = Result<Token, ()>> {
        use std::iter::from_fn;

        let mut lexer = self.lex();
        from_fn(move || {
            let kind = lexer.next()?;
            let line = lexer.extras.0 as u32 + 1;
            // this handles the edge case where the `kind` is `Ok(NewLine)`
            // todo: where to locate newlines?
            let column = lexer.span().start.saturating_sub(lexer.extras.1) as u32 + 1;
            let slice = self.slice(lexer.span());
            Some(kind.map(|kind| Token {
                kind,
                line,
                column,
                slice,
            }))
        })
    }
}

#[cfg(test)]
mod test;
