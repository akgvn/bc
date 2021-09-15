use crate::compiler::Instruction;
use std::collections::HashMap;

pub struct Vm<'source> {
    ops: Vec<Instruction<'source>>,
    env: &'source mut HashMap<String, f64>, // NOTE: The map lives longer than the 'source, but the vm doesn't.
}

impl<'source> Vm<'source> {
    pub fn new(
        mut ops: Vec<Instruction<'source>>,
        env: &'source mut HashMap<String, f64>,
        // fn_env: &'source mut HashMap<String, f64>, // TODO add an environment for functions? Also check for arity of functions too!
    ) -> Vm<'source> {
        ops.reverse();
        Self { ops, env }
    }

    pub fn interpret(&mut self) {
        let mut stack: Vec<f64> = vec![];

        let pop = |stack: &mut Vec<f64>| -> f64 {
            match stack.pop() {
                Some(value) => value,
                None => {
                    panic!("Error: No value on the stack! Must be a problem with the parser?");
                }
            }
        };

        loop {
            let operation = self.ops.pop();
            if operation.is_none() {
                break;
            }
            let operation = operation.unwrap();
            match operation {
                Instruction::Add => {
                    let a = pop(&mut stack);
                    let b = pop(&mut stack);

                    stack.push(a + b);
                }
                Instruction::Sub => {
                    let a = pop(&mut stack);
                    let b = pop(&mut stack);

                    stack.push(a - b);
                }
                Instruction::Negate => {
                    let num = pop(&mut stack);

                    stack.push(-num);
                }
                Instruction::Mult => {
                    let a = pop(&mut stack);
                    let b = pop(&mut stack);

                    stack.push(a * b);
                }
                Instruction::Div => {
                    let a = pop(&mut stack);
                    let b = pop(&mut stack);

                    stack.push(a / b);
                }
                Instruction::Mod => {
                    let a = pop(&mut stack);
                    let b = pop(&mut stack);

                    stack.push(a % b);
                }
                Instruction::PushConstant(num) => {
                    stack.push(num);
                }
                Instruction::GetVal(val_ident) => match self.env.get(val_ident) {
                    Some(val) => stack.push(*val),
                    None => error("Non-existent variable."),
                }
                Instruction::Assign(val_ident) => {
                    let val = pop(&mut stack);
                    self.env.insert(String::from(val_ident), val);
                }
                Instruction::CallFn(fn_name) => {
                    // TODO Add the ability to define functions
                    // and check against the definitions here.
                    let val = pop(&mut stack);

                    if fn_name == "sin" {
                        let result = val.sin();
                        stack.push(result);
                    } else if fn_name == "cos" {
                        let result = val.cos();
                        stack.push(result);
                    } else if fn_name == "sqrt" {
                        let result = val.sqrt();
                        stack.push(result);
                    } else {
                        error("Unknown function name!");
                    }
                }
            }
        }
        let val = stack.pop();
        if val.is_some() {
            println!("{}", val.unwrap());
        }
    }
}

fn error(error_text: &str) {
    println!("Error: {}", error_text);
}
