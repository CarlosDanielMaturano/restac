use crate::tokens::Token;

pub struct Evaluator {
    pub rpn_tokens: Vec<Token>,
}

impl Evaluator {
    pub fn new(rpn_tokens: Vec<Token>) -> Evaluator {
        Evaluator {  rpn_tokens }
    }
    pub fn evaluate(&self) -> f64 {
        let mut stack: Vec<f64> = Vec::new();
        self.rpn_tokens.iter().for_each(|token| {
            match token {
                Token::LITERAL(value) => stack.push(value.trim().parse::<f64>().expect("")),
                Token::PLUS => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b)
                }
                Token::MINUS => {
                    let b: f64 = stack.pop().unwrap();
                    let a: f64 = stack.pop().unwrap();
                    stack.push(a-b);
                }
                Token::MUTIPLY=> {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a *  b)
                }
                Token::DIVIDE=> {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a / b)
                }
                _ => {}
            }
        });
        stack.pop().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod test {
    use super::Evaluator;
    use super::super::{lexer::Lexer, parser::Parser};
    #[test]
    fn evaluator_test_0() {
        let mut lexer = Lexer::new("3 + 4 * 5".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), 23.0);
    }
    #[test]
    fn evaluator_test_1() {
        let mut lexer = Lexer::new("(3 + 4) * 5".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), 35.0);
    }
    #[test]
    fn evaluator_test_2() {
        let mut lexer = Lexer::new("3 - 4".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), -1.0);
    }
    #[test]
    fn evaluator_test_3() {
        let mut lexer = Lexer::new("3 * (0 - 4)".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), -12.0);
    }
}
