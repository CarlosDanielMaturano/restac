use math_parser::*;

fn eval_operation(input: &String) -> f64 {
    let mut lexer = lexer::Lexer::new(input.to_string());
    let parser = parser::Parser::new(lexer.gen_tokens());
    let evaluator = evaluator::Evaluator::new(parser.gen_rpn_tokens());
    let result = evaluator.evaluate();
    dbg!(result);
    result
}

fn main() -> ! {
    loop {
        println!("math input >");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf);
        let result = eval_operation(&buf);
        println!("Result is: {}", result);
    }
}
