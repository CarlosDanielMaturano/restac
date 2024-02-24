use super::tokens::Token;

pub struct Lexer {
    pub input: String,
    pub char: Option<char>,
    pub index: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.split_whitespace().collect::<String>(),
            char: Some(input.chars().nth(0).expect("")),
            index: 1,
        }
    }
    pub fn advance(&mut self) {
        if self.index >= self.input.len() {
            self.char = None;
            return
        }
        self.char = self.input.
            chars()
            .nth(self.index);
        self.index+= 1;
    }
    pub fn gen_number(&mut self) -> Token {
        let mut number = String::new();
        let mut dot_count: usize = 0;
        while let Some(char) = self.char {
            if !char.is_numeric() && char != '.' {
                break
            }
            if char.is_numeric() {
                number += &char.to_string();
                self.advance();
                continue;
            }
            dot_count += 1;
            if dot_count >= 2 {
                panic!("Invalid literal")
            }
            number += &'.'.to_string();
            self.advance()
        }
        Token::LITERAL(number)
    }
    pub fn gen_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(char) = self.char {
            if char.is_numeric() {
                tokens.push(self.gen_number());
                continue
            }
            match char {
                '+' => tokens.push(Token::PLUS),
                '-' => tokens.push(Token::MINUS),
                '*' => tokens.push(Token::MUTIPLY),
                '/' => tokens.push(Token::DIVIDE),
                '(' => tokens.push(Token::LPAREN),
                ')' => tokens.push(Token::RPAREN),
                _ => {
                    panic!("Unrecognized char {}", char)
                }
            }
            self.advance();
        }
        tokens
    }
}

#[cfg(test)]
mod test {
    use super::{Token, Lexer};
    #[test]
    fn lexer_test_0() {
        let mut lexer = Lexer::new(String::from("3 + 4 - 5"));
        let tokens = vec![
            Token::LITERAL(String::from("3")),
            Token::PLUS,
            Token::LITERAL(String::from("4")),
            Token::MINUS,
            Token::LITERAL(String::from("5")),
        ];
        assert_eq!(tokens, lexer.gen_tokens());
    }
    #[test]
    fn lexer_test_1() {
        let mut lexer = Lexer::new(String::from("(3 - 4.5) - ( 7 * 6 - 4)"));
        let tokens = vec![
            Token::LPAREN,
            Token::LITERAL("3".to_string()),
            Token::MINUS,
            Token::LITERAL("4.5".to_string()),
            Token::RPAREN,
            Token::MINUS,
            Token::LPAREN,
            Token::LITERAL("7".to_string()),
            Token::MUTIPLY,
            Token::LITERAL("6".to_string()),
            Token::MINUS,
            Token::LITERAL("4".to_string()),
            Token::RPAREN,
        ];
        assert_eq!(tokens, lexer.gen_tokens());
    }
    #[test]
    #[should_panic]
    fn lexer_panic_1() {
        let mut lexer = Lexer::new(String::from("3 + 4..5"));
        lexer.gen_tokens();
    }
    #[test]
    #[should_panic]
    fn lexer_panic_2() {
        let mut lexer = Lexer::new(String::from("3 # 4"));
        lexer.gen_tokens();
    }
}

