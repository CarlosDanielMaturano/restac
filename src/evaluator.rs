use crate::tokens::Token;

pub struct Evaluator {
    pub rpn_tokens: Vec<Token>,
    pub stack: Vec<f64>,
}

impl Evaluator {
    pub fn new(rpn_tokens: Vec<Token>) -> Evaluator {
        Evaluator {
            rpn_tokens,
            stack: Vec::new(),
        }
    }

    pub fn apply_operation_to_stack<F>(&mut self, operation: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        let b = self.stack.pop().expect("Cannot pop out of stack");
        let a = self.stack.pop().expect("Cannot pop out of stack");
        self.stack.push(operation(a, b));
    }

    pub fn evaluate(&mut self) -> f64 {
        self.rpn_tokens.clone().iter().for_each(|token| match token {
            Token::LITERAL(value) => self.stack.push(value.trim().parse::<f64>().expect("")),
            Token::PLUS => self.apply_operation_to_stack(|a, b| a + b),
            Token::MINUS => self.apply_operation_to_stack(|a, b| a - b),
            Token::MUTIPLY => self.apply_operation_to_stack(|a, b| a * b),
            Token::DIVIDE =>  self.apply_operation_to_stack(|a, b| a / b),
            _ => {}
        });
        self.stack.pop().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod test {
    use super::super::{lexer::Lexer, parser::Parser};
    use super::Evaluator;
    #[test]
    fn evaluator_test_0() {
        let mut lexer = Lexer::new("3 + 4 * 5".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let mut evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), 23.0);
    }
    #[test]
    fn evaluator_test_1() {
        let mut lexer = Lexer::new("(3 + 4) * 5".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let mut evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), 35.0);
    }
    #[test]
    fn evaluator_test_2() {
        let mut lexer = Lexer::new("3 - 4".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let mut evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), -1.0);
    }
    #[test]
    fn evaluator_test_3() {
        let mut lexer = Lexer::new("3 * (0 - 4)".to_string());
        let parser = Parser::new(lexer.gen_tokens());
        let mut evaluator = Evaluator::new(parser.gen_rpn_tokens());
        assert_eq!(evaluator.evaluate(), -12.0);
    }
}
