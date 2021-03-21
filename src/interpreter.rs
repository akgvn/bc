use crate::parser::Op;

pub struct Vm {
    ops: Vec<Op>,
}

impl Vm {
    pub fn new(mut ops: Vec<Op>) -> Vm {
        ops.reverse();
        Self { ops }
    }

    pub fn interpret(&mut self) {
        let val: f64;
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
            }
        }
        val = stack.pop().unwrap();
        println!("{}", val);
    }
}
