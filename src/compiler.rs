#[derive(Debug)]
pub enum Op<'source> {
    Add,
    Sub,
    Negate,
    Mult,
    Div,
    GetVal(&'source str),
    Assign(&'source str),
    PushConstant(f64),
}
