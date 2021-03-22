use crate::parser::Op;
use std::collections::HashMap;

pub struct Vm<'source> {
    ops: Vec<Op<'source>>,
    env: &'source mut HashMap<String, f64>, // NOTE: The map lives longer than the 'source, but the vm doesn't.
}

impl<'source> Vm<'source> {
    pub fn new(mut ops: Vec<Op<'source>>, env: &'source mut HashMap<String, f64>) -> Vm<'source> {
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
                Op::Add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                }
                Op::Sub => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a - b);
                }
                Op::Negate => {
                    let num = stack.pop().unwrap();
                    stack.push(-num);
                }
                Op::Mult => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                }
                Op::Div => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a / b);
                }
                Op::Constant(num) => {
                    stack.push(num);
                }
                Op::GetVal(val_ident) => match self.env.get(val_ident) {
                    Some(val) => {
                        stack.push(*val);
                    }
                    None => {
                        panic!("Non-existent variable.");
                    }
                },
                Op::Assign(val_ident) => {
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
