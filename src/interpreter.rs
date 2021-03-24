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
    ) -> Vm<'source> {
        ops.reverse();
        Self { ops, env }
    }

    pub fn interpret(&mut self) {
        let mut stack: Vec<f64> = vec![];
        loop {
            let operation = self.ops.pop();
            if operation.is_none() {
                break;
            }
            let operation = operation.unwrap();
            match operation {
                Instruction::Add => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                }
                Instruction::Sub => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a - b);
                }
                Instruction::Negate => {
                    let num = stack.pop().unwrap();
                    stack.push(-num);
                }
                Instruction::Mult => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a * b);
                }
                Instruction::Div => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a / b);
                }
                Instruction::PushConstant(num) => {
                    stack.push(num);
                }
                Instruction::GetVal(val_ident) => match self.env.get(val_ident) {
                    Some(val) => {
                        stack.push(*val);
                    }
                    None => {
                        error("Non-existent variable.");
                    }
                },
                Instruction::Assign(val_ident) => {
                    let val = stack.pop().unwrap();
                    self.env.insert(String::from(val_ident), val);
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
