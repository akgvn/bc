mod tokenizer;
use std::io::{stdin, stdout, Write};
use tokenizer::*;

fn main() {
    let mut user_input = String::new();
    loop {
        print!("> ");
        let _ = stdout().flush();

        stdin().read_line(&mut user_input).expect(
            "The input is weeeirrrrdddd. Probably not UTF-8. Use only English characters for now.",
        );
        println!("You typed: {}", user_input);

        if user_input.contains("exit") {
            break;
        }

        let tokens = tokens_from_text(&user_input);
        println!("{:?}", tokens);
        user_input.clear();
    }
}
