use math_parser::*;

fn eval_operation(input: &String) -> f64 {
    let mut lexer = lexer::Lexer::new(input.to_string());
    let parser = parser::Parser::new(lexer.gen_tokens());
    let mut evaluator = evaluator::Evaluator::new(parser.gen_rpn_tokens());
    let result = evaluator.evaluate();
    result
}

fn main() -> ! {
    loop {
        println!("expression>");
        let mut buf = String::new();
        if let Err(err) = std::io::stdin().read_line(&mut buf) {
            println!("A error ocurred while reading the input: {}", err);
        };
        let result = eval_operation(&buf);
        println!("{result}");
    }
}
