mod interpreter;
mod parser;
mod tokenizer;

use interpreter::*;
use parser::*;
use std::io::{stdin, stdout, Write};
use tokenizer::*;

fn main() {
    let mut user_input = String::new();

    println!("-- rust bc clone 0.0.1 --");

    loop {
        print!("> ");
        let _ = stdout().flush();

        stdin()
            .read_line(&mut user_input)
            .expect("The input is weeeirrrrdddd. Use only ASCII characters for now.");
        // println!("You typed: {}", user_input);

        // TODO This should be a intrinsic function at some point.
        if user_input.contains("quit") {
            break;
        }

        let tokens = tokens_from_text(&user_input);
        // println!("Tokens: {:?}", tokens);

        let ops = get_operations(tokens);

        // println!("Ops: {:?}", ops);

        let mut vm = Vm::new(ops);
        vm.interpret();

        user_input.clear();
    }
}
