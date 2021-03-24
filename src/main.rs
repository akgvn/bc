mod compiler;
mod interpreter;
mod parser;
mod tokenizer;

use crate::compiler::*;
use crate::interpreter::*;
use crate::parser::*;
use crate::tokenizer::*;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut user_input = String::new();
    let mut map: HashMap<String, f64> = HashMap::new();
    map.insert(String::from("debug"), 5.0);

    println!("< bc-r: a bc clone - 0.0.1 >");

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

        if map["debug"] > 0.5 {
            println!("Tokens: {:?}", tokens);
        }

        let parser = Parser::new(tokens);
        let ast = parser.parse();

        println!("{}", ast);

        /*
        let (ops, syntax_error) = get_operations(tokens);

        if map["debug"] > 0.5 {
            println!("Ops: {:?}", ops);
        }

        if !syntax_error {
            let mut vm = Vm::new(ops, &mut map);
            vm.interpret();
        }
        */

        user_input.clear();
    }
}
