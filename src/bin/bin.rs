use math_parser::*;

fn eval_operation(input: String) -> f64 {
    let mut lexer = lexer::Lexer::new(input);
    let parser = parser::Parser::new(lexer.gen_tokens());
    let evaluator = evaluator::Evaluator::new(parser.gen_rpn_tokens());
    evaluator.evaluate()
}

fn main() -> ! {
    loop {}
}
