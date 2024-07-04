// https://icfpcontest2024.github.io/icfp.html

use crate::prelude::*;

pub struct Program(Vec<u8>);

// TODO: u64 is not enough?
type Number = u64;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    // * Primitives
    // T or F
    Bool(bool),
    // I
    Int(Number),
    // S
    S(Vec<u8>),

    // * Unarray
    // U -
    Negate(Box<Token>),
    // U !
    Not(Box<Token>),
    // U #
    StringToInt(Box<Token>),
    // U $
    IntToString(Box<Token>),

    // * Binary
    // B +
    Add(Box<Token>, Box<Token>),
    // B -
    Sub(Box<Token>, Box<Token>),
    // B *
    Mul(Box<Token>, Box<Token>),
    // B /
    Div(Box<Token>, Box<Token>),
    // B %,
    Mod(Box<Token>, Box<Token>),
    // B <
    Lt(Box<Token>, Box<Token>),
    // B >
    Gt(Box<Token>, Box<Token>),
    // B =
    Eq(Box<Token>, Box<Token>),
    // B |
    Or(Box<Token>, Box<Token>),
    // B &
    And(Box<Token>, Box<Token>),
    // B .
    Concat(Box<Token>, Box<Token>),
    // B T
    Take(Box<Token>, Box<Token>),
    // B D
    Drop(Box<Token>, Box<Token>),
    // B $
    Apply(Box<Token>, Box<Token>),

    // * If
    // ?
    If(Box<Token>, Box<Token>, Box<Token>),

    // * Lambda
    // L
    Lambda(Number),
    // v
    Var(Number),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            S(s) => {
                write!(f, "{}", convert_str(&s))
            }
            _ => {
                todo!();
            }
        }
    }
}

fn parse_int(body: &[u8]) -> Number {
    todo!()
}

// abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~<space><newline>

// 33 to 126
// = 126 - 33 => 93

fn convert_str(body: &[u8]) -> String {
    lazy_static! {
        static ref CONVERT: &'static [u8; 94] = byte_strings::concat_bytes!(br##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ "##, b"\n");
    }
    let bs: Vec<u8> = body.iter().map(|b| CONVERT[(b - 33) as usize]).collect();
    String::from_utf8(bs).unwrap()
}

fn parse(src: &[Vec<u8>]) -> Result<Token> {
    assert!(!src.is_empty());
    let first = src.first().unwrap();
    let remain = &src[1..];

    let indicator = first[0];
    let body = &first[1..];
    let token = match indicator {
        b'T' => Token::Bool(true),
        b'F' => Token::Bool(false),
        b'I' => Token::Int(parse_int(body)),
        b'S' => Token::S(body.to_vec()),
        _ => todo!(),
    };
    Ok(token)
}

impl Token {
    fn eval(&self) -> Token {
        todo!();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_test() -> Result<()> {
        let src = vec![b"SB%,,/".to_vec()];
        let program = parse(&src)?;

        assert_eq!(program, Token::S(b"B%,,/".to_vec()));
        assert_eq!(program.to_string(), "Hello");

        Ok(())
    }
}
