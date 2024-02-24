use super::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }
    pub fn gen_rpn_tokens(&self) -> Vec<Token> {
        let mut op_stack: Vec<Token> = Vec::new();
        let mut result: Vec<Token> = Vec::new();
        self.tokens.iter().for_each(|token| {
            let token = token.clone();
            match token {
                Token::LITERAL(_) => result.push(token),
                Token::LPAREN => op_stack.push(token),
                Token::RPAREN => {
                    loop {
                        if let Some(Token::LPAREN) = op_stack.last() {
                            break;
                        }
                        result.push(op_stack.pop().expect("Cannot pop out of the stack"));
                    }
                    op_stack.pop().expect("Cannot pop out of the stack");
                }
                _ => {
                    while !op_stack.is_empty()
                        && get_precedence(&token) <= get_precedence(op_stack.last().expect("Cannot pop out of the stack"))
                    {
                        result.push(op_stack.pop().expect("Cannot pop out of the stack"));
                    }
                    op_stack.push(token)
                }
            }
        });
        op_stack.reverse();
        result.append(&mut op_stack);
        result
    }
}

fn get_precedence(token: &Token) -> usize {

    match *token {
        Token::MUTIPLY | Token::DIVIDE => 2,
        Token::PLUS | Token::MINUS => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::{Parser, Token};
    use super::super::lexer::Lexer;
    #[test]
    fn parser_test_0() {
        let mut lexer = Lexer::new("3 + 4 * 2".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let tokens = vec![
            Token::LITERAL("3".to_string()),
            Token::LITERAL("4".to_string()),
            Token::LITERAL("2".to_string()),
            Token::MUTIPLY,
            Token::PLUS,
        ];
        assert_eq!(tokens, parser.gen_rpn_tokens());
    }
    #[test]
    fn parser_test_1() {
        let mut lexer = Lexer::new("(3 + 4) * 2".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let tokens = vec![
            Token::LITERAL("3".to_string()),
            Token::LITERAL("4".to_string()),
            Token::PLUS,
            Token::LITERAL("2".to_string()),
            Token::MUTIPLY,
        ];
        assert_eq!(tokens, parser.gen_rpn_tokens());
    }
    #[test]
    fn parser_test_2() {
        let mut lexer = Lexer::new("4*3+5*9/2".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let tokens = vec![
            Token::LITERAL("4".to_string()),
            Token::LITERAL("3".to_string()),
            Token::MUTIPLY,
            Token::LITERAL("5".to_string()),
            Token::LITERAL("9".to_string()),
            Token::MUTIPLY,
            Token::LITERAL("2".to_string()),
            Token::DIVIDE,
            Token::PLUS,
        ];
        assert_eq!(tokens, parser.gen_rpn_tokens());
    }
    #[test]
    fn parser_test_3() {
        let mut lexer = Lexer::new("3 - 4".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let tokens = vec![
            Token::LITERAL("3".to_string()),
            Token::LITERAL("4".to_string()),
            Token::MINUS,
        ];
        assert_eq!(tokens, parser.gen_rpn_tokens());
    }
    #[test]
    fn parser_test_4() {
        let mut lexer = Lexer::new("3 - 4 - 7".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let tokens = vec![
            Token::LITERAL("3".to_string()),
            Token::LITERAL("4".to_string()),
            Token::MINUS,
            Token::LITERAL("7".to_string()),
            Token::MINUS,
        ];
        assert_eq!(tokens, parser.gen_rpn_tokens());
    }
}
