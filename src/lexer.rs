use crate::token::Token;

struct Lexer {
    input: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        todo!()
    }

    pub fn next_token(&self) -> Token {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenKind};

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input: &str = "=+(){},;";

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
        ];

        let lexer = Lexer::new(input);

        for (idx, expected_token) in expected.into_iter().enumerate() {
            let received_token = lexer.next_token();
            assert_eq!(
                expected_token.kind, received_token.kind,
                "test[{idx}] - token type wrong. expected {:?}, got {:?}",
                expected_token.kind, received_token.kind
            );
            assert_eq!(
                expected_token.literal, received_token.literal,
                "tests[{idx}] - literal wrong. expected {}, got {}",
                expected_token.literal, received_token.literal
            );
        }
    }
}
