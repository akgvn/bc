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

use std::env;
use std::fs;

fn main() {
    let mut user_input = String::new();
    let mut map: HashMap<String, f64> = HashMap::new();
    map.insert(String::from("debug"), 0.0);
    println!("< bc-r: a bc clone - 0.0.3 >");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // First arg is executable location.
        let contents = fs::read_to_string(&args[1]).expect("Couldn't read the source file.");
        user_input.push_str(&contents);
    }

    loop {
        if !(user_input.len() > 0) {
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
        }

        let tokens = tokens_from_text(&user_input);

        if map["debug"] > 0.5 {
            println!("Tokens: {:?}", tokens);
        }

        let parser = Parser::new(tokens);
        let statements = parser.parse();

        let mut ops = vec![];
        for ast in statements {
            if map["debug"] > 0.5 {
                println!("AST: {}", ast);
            }

            let compiler = Compiler::new(); // I don't like doing this every loop. TODO
            ops.append(&mut compiler.compile(ast));
        }

        if map["debug"] > 0.5 {
            println!("Ops: {:?}", ops);
        }

        let mut vm = Vm::new(ops, &mut map);
        vm.interpret();

        user_input.clear();
    }
}
