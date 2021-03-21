mod tokenizer;
use std::io::{stdin, stdout, Write};
use tokenizer::*;

fn main() {
    let mut user_input = String::new();
    loop {
        print!("> ");
        let _ = stdout().flush();

        stdin()
            .read_line(&mut user_input)
            .expect("The input is weeeirrrrdddd.");
        println!("You typed: {}", user_input);

        if user_input.contains("exit") {
            break;
        }

        let mut tk = Tokenizer::new(&user_input);
        tk.tokenize();
        let tokens = tk.get_tokens();
        println!("{:?}", tokens);
        user_input.clear();
    }
}
