use crate::parser::AstNode;
use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Instruction<'source> {
    Add,
    Sub,
    Negate,
    Mult,
    Div,
    Mod,
    Pow,
    GetVal(&'source str),
    Assign(&'source str),
    PushConstant(f64),
    // TODO PopAndAssign(&'source str), // For function arguments.
    CallFn(&'source str)
}

pub struct Compiler<'source> {
    operations: Vec<Instruction<'source>>,
}

impl<'source> Compiler<'source> {
    pub fn new() -> Self {
        Self { operations: vec![] }
    }

    pub fn compile(mut self, ast_root: AstNode<'source>) -> Vec<Instruction<'source>> {
        self.compile_ast_node(ast_root, false);
        self.operations
    }

    fn compile_ast_node(&mut self, node: AstNode<'source>, is_equals: bool) {
        match node {
            AstNode::Ident(ident) => {
                if is_equals {
                    self.operations.push(Instruction::Assign(ident));
                } else {
                    self.operations.push(Instruction::GetVal(ident));
                }
            }
            AstNode::Number(number) => {
                self.operations.push(Instruction::PushConstant(number));
            }
            AstNode::Op(op_token, children_nodes) => {
                let mut equals = false;
                let child_count = children_nodes.len();
                let is_negation = op_token == Token::Minus && child_count == 1;
                if op_token == Token::Equals {
                    equals = true;
                }

                for child_node in children_nodes.into_iter().rev() {
                    self.compile_ast_node(child_node, equals);
                }

                if is_negation {
                    // Push negate
                    self.operations.push(Instruction::Negate);
                } else if op_token == Token::Plus && child_count == 1 {
                    // Ignore plus
                } else if let Token::FnCall(fn_name) = op_token {
                    // Handle function call
                    // Don't forget that the arguments get pushed to stack in reverse.
                    self.operations.push(Instruction::CallFn(fn_name));
                } else if !equals {
                    self.push_op(op_token);
                }
            }
        }
    }

    fn push_op(&mut self, op_token: Token) {
        match op_token {
            Token::Plus    => self.operations.push(Instruction::Add),
            Token::Minus   => self.operations.push(Instruction::Sub),
            Token::Star    => self.operations.push(Instruction::Mult),
            Token::Slash   => self.operations.push(Instruction::Div),
            Token::Percent => self.operations.push(Instruction::Mod),
            Token::Power   => self.operations.push(Instruction::Pow),
            _ => {
                panic!("Unexpected token!");
            }
        }
    }
}
