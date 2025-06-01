use crate::token::{Literal, Token, TokenType};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
            keywords: keywords,
        }
    }

    pub fn show_self(&self) -> (&String, &Vec<Token>, &usize, &usize, &usize, &bool) {
        return (
            &self.source,
            &self.tokens,
            &self.start,
            &self.current,
            &self.line,
            &self.has_error,
        );
    }

    // 遍历文章全部token
    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.current_at_end() {
            self.scan_token();
        }
        &self.tokens
    }

    // capture the single token
    fn scan_token(&mut self) {
        let text_char = self.current_char();
        self.start = self.current;
        self.consume_char();

        match text_char {
            // match a single character
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),
            '.' => self.add_token(TokenType::Dot, Literal::None),
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '+' => self.add_token(TokenType::Semicolon, Literal::None),
            ';' => self.add_token(TokenType::Semicolon, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),

            // conditional comsume two charafcter
            '!' => match self.peek_if_two_char_symbol('=') {
                true => self.add_token(TokenType::BangEqual, Literal::None),
                false => self.add_token(TokenType::Bang, Literal::None),
            },
            '=' => match self.peek_if_two_char_symbol('=') {
                true => self.add_token(TokenType::EqualEqual, Literal::None),
                false => self.add_token(TokenType::Equal, Literal::None),
            },
            '>' => match self.peek_if_two_char_symbol('=') {
                true => self.add_token(TokenType::GreaterEqual, Literal::None),
                false => self.add_token(TokenType::Greater, Literal::None),
            },
            '<' => match self.peek_if_two_char_symbol('=') {
                true => self.add_token(TokenType::LessEqual, Literal::None),
                false => self.add_token(TokenType::LessEqual, Literal::None),
            },

            '/' => self.add_token(TokenType::Slash, Literal::None),

            // newline and whitespace
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,

            // string literals
            '"' => self.add_string(),
            _ => match self.current_char() {
                c if c.is_ascii_digit() => self.add_number(),
                c if c.is_ascii_alphabetic() || c=='_' =>self.add_identifier(),
                _ => {
                    eprintln!("{}: Unexpected character.", self.line);
                    self.has_error = true;
                }
            },
        }
    }

    fn peek_if_two_char_symbol(&mut self, expected: char) -> bool {
        if self.current_at_end() {
            // determine whether in the end
            return false;
        }
        let match_res = self.current_char() == expected;
        if match_res {
            self.consume_char();
        }
        match_res
    }
    fn add_token(&mut self, tokentype: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type: tokentype,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        })
    }
    fn add_identifier(&mut self) {
        while self.current_char().is_alphanumeric() || self.current_char() == '_' {
            self.current += 1;
        }
        let text = self.source.get(self.start..self.current).unwrap();
        match self.keywords.get(text) {
            Some(token_type) => self.add_token(token_type.clone(), Literal::None),
            None => self.add_token(TokenType::Identifier, Literal::None),
        }
    }
    fn add_number(&mut self) {
        self.start = self.current;
        while self.current_char().is_ascii_digit() {
            self.consume_char();
        }
        if self.current_char() == '.' && self.next_char().is_ascii_digit() {
            self.consume_char();
        }
        while self.current_char().is_ascii_digit() {
            self.consume_char();
        }
        let value = (&self.source[self.start..self.current])
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::Number, Literal::Number(value));
    }
    fn add_string(&mut self) {
        // now the current is pointing the char "
        //the start point the first char " in string
        while self.current_char() != '"' {
            if self.current_char() == '\n' {
                self.line += 1;
            }
            self.consume_char();
        }
        // now the current is pointing the final char " in string
        if self.current_at_end() {
            eprintln!("{}:Unterminated string.", self.line);
            self.has_error = true;
        }

        self.consume_char();
        // now the current is pointing the char after "
        let value = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .to_string();
        self.add_token(TokenType::String, Literal::String(value));
    }
    fn consume_char(&mut self) {
        //cthe current point the next char
        self.current += 1;
    }

    fn current_char(&self) -> char {
        //  return the char pointed to by current
        self.source.as_bytes()[self.current] as char
    }
    fn next_char(&self) -> char {
        self.source.as_bytes()[self.current + 1] as char
    }
    fn current_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod test_scanner {
    use super::*;
    use std::vec;
    #[test]
    fn test_new() {
        let source_str = String::from("hello world");
        let scanner = Scanner::new(source_str);
        let (source, tokens, start, current, line, has_error) = scanner.show_self();
        assert_eq!(*source, "hello world".to_string());
        assert_eq!(*tokens, vec![]);
        assert_eq!(*start, 0);
        assert_eq!(*current, 0);
        assert_eq!(*line, 1);
        assert_eq!(*has_error, false);
    }
    #[test]
    fn test_scan_one_char() {
        let source_str = String::from("(");
        let mut scanner = Scanner::new(source_str);
        scanner.scan_tokens();
        let (_, tokens, start, current, line, has_error) = scanner.show_self();
        let token_list = vec![Token {
            token_type: TokenType::LeftParen,
            lexeme: "(".to_string(),
            literal: Literal::None,
            line: 1,
        }];
        assert_eq!(*tokens, token_list);
        assert_eq!(*start, 0);
        assert_eq!(*current, 1);
        assert_eq!(*line, 1);
        assert_eq!(*has_error, false);
    }
    #[test]
    fn test_scan_two_char() {
        let source_str = String::from(" >= ");
        let mut scanner = Scanner::new(source_str);
        scanner.scan_tokens();
        let (_, tokens, start, current, line, has_error) = scanner.show_self();
        let token_list = vec![Token {
            token_type: TokenType::GreaterEqual,
            lexeme: ">=".to_string(),
            literal: Literal::None,
            line: 1,
        }];
        assert_eq!(*tokens, token_list);
        assert_eq!(*start, 3);
        assert_eq!(*current, 4);
        assert_eq!(*line, 1);
        assert_eq!(*has_error, false);
    }
    #[test]
    fn test_add_string() {
        let source_str = String::from(" \"hello\" ");
        let mut scanner = Scanner::new(source_str);
        scanner.scan_tokens();
        let (_, tokens, start, current, line, has_error) = scanner.show_self();
        let token_list = vec![Token {
            token_type: TokenType::String,
            lexeme: "\"hello\"".to_string(),
            literal: Literal::String("\"hello\"".to_string()),
            line: 1,
        }];
        assert_eq!(*tokens, token_list);
        assert_eq!(*start, 8);
        assert_eq!(*current, 9);
        assert_eq!(*line, 1);
        assert_eq!(*has_error, false);
    }

    #[test]
    fn test_add_string2() {
        let source_str = String::from(" \"hello\"");
        let mut scanner = Scanner::new(source_str);
        scanner.scan_tokens();
        let (_, tokens, start, current, line, has_error) = scanner.show_self();
        let token_list = vec![Token {
            token_type: TokenType::String,
            lexeme: "\"hello\"".to_string(),
            literal: Literal::String("\"hello\"".to_string()),
            line: 1,
        }];
        assert_eq!(*tokens, token_list);
        assert_eq!(*start, 1);
        assert_eq!(*current, 8);
        assert_eq!(*line, 1);
        assert_eq!(*has_error, false);
    }

    #[test]
    fn test_add_number() {
        let source_str = String::from("123.456");
        let mut scanner = Scanner::new(source_str);
        scanner.scan_tokens();
        let (_, tokens, _, _, _, _) = scanner.show_self();
        let token_list = vec![Token {
            token_type: TokenType::Number,
            lexeme: "123.456".to_string(),
            literal: Literal::Number(123.456),
            line: 1,
        }];
        assert_eq!(*tokens, token_list);
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers() {
        let mut scanner = Scanner::new(String::from("andy formless fo _ _123 _abc ab123 \n abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_"));
        let tokens = scanner.scan_tokens();

        let expected_tokens = [
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("andy"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("formless"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("fo"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("_"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("_123"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("_abc"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("ab123"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from(
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
                ),
                literal: Literal::None,
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                literal: Literal::None,
                line: 2,
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i]);
        }
    }

    #[test]
    fn keywords() {
        let mut scanner = Scanner::new(String::from(
            "and class else false for fun if nil or return super this true var while",
        ));
        let tokens = scanner.scan_tokens();

        let expected_tokens = [
            Token {
                token_type: TokenType::And,
                lexeme: String::from("and"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Class,
                lexeme: String::from("class"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Else,
                lexeme: String::from("else"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::False,
                lexeme: String::from("false"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::For,
                lexeme: String::from("for"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Fun,
                lexeme: String::from("fun"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::If,
                lexeme: String::from("if"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Nil,
                lexeme: String::from("nil"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Or,
                lexeme: String::from("or"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Return,
                lexeme: String::from("return"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Super,
                lexeme: String::from("super"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::This,
                lexeme: String::from("this"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::True,
                lexeme: String::from("true"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Var,
                lexeme: String::from("var"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::While,
                lexeme: String::from("while"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                literal: Literal::None,
                line: 1,
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i]);
        }
    }

    #[test]
    fn numbers() {
        let mut scanner = Scanner::new(String::from("123\n123.456\n.456\n123."));
        let tokens = scanner.scan_tokens();

        let expected_tokens = [
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("123"),
                literal: Literal::Number(123.0),
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("123.456"),
                literal: Literal::Number(123.456),
                line: 2,
            },
            Token {
                token_type: TokenType::Dot,
                lexeme: String::from("."),
                literal: Literal::None,
                line: 3,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("456"),
                literal: Literal::Number(456.0),
                line: 3,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("123"),
                literal: Literal::Number(123.0),
                line: 4,
            },
            Token {
                token_type: TokenType::Dot,
                lexeme: String::from("."),
                literal: Literal::None,
                line: 4,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                literal: Literal::None,
                line: 4,
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i]);
        }
    }

    #[test]
    fn punctuators() {
        let mut scanner = Scanner::new(String::from("(){};,+-*!===<=>=!=<>/."));
        let tokens = scanner.scan_tokens();

        let expected_tokens = [
            Token {
                token_type: TokenType::LeftParen,
                lexeme: String::from("("),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                lexeme: String::from(")"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::LeftBrace,
                lexeme: String::from("{"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::RightBrace,
                lexeme: String::from("}"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: String::from(";"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: String::from("+"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Star,
                lexeme: String::from("*"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::BangEqual,
                lexeme: String::from("!="),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: String::from("=="),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::LessEqual,
                lexeme: String::from("<="),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::GreaterEqual,
                lexeme: String::from(">="),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::BangEqual,
                lexeme: String::from("!="),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Less,
                lexeme: String::from("<"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Greater,
                lexeme: String::from(">"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: String::from("/"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Dot,
                lexeme: String::from("."),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::from(""),
                literal: Literal::None,
                line: 1,
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i]);
        }
    }

    #[test]
    fn strings() {
        let mut scanner = Scanner::new(String::from("\"\" \n \"string\""));
        let tokens = scanner.scan_tokens();

        let expected_tokens = [
            Token {
                token_type: TokenType::String,
                lexeme: String::from("\"\""),
                literal: Literal::String(String::from("")),
                line: 1,
            },
            Token {
                token_type: TokenType::String,
                lexeme: String::from("\"string\""),
                literal: Literal::String(String::from("string")),
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::from(""),
                literal: Literal::None,
                line: 2,
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i]);
        }
    }

    #[test]
    fn whitespace() {
        let mut scanner = Scanner::new(String::from(
            "space    tabs				newlines




        end",
        ));
        let tokens = scanner.scan_tokens();

        let expected_tokens = [
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("space"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("tabs"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("newlines"),
                literal: Literal::None,
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("end"),
                literal: Literal::None,
                line: 6,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::from(""),
                literal: Literal::None,
                line: 6,
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i]);
        }
    }
}

