#[derive(Debug, PartialEq, Clone)]
pub enum MathExpression<'a> {
    Mi(&'a str),
    Mo(&'a str),
    Mn(&'a str),
    Msqrt(Box<MathExpression<'a>>),
    Mrow(Vec<MathExpression<'a>>),
    Mfrac(Box<MathExpression<'a>>, Box<MathExpression<'a>>),
    Msup(Box<MathExpression<'a>>, Box<MathExpression<'a>>),
    Msub(Box<MathExpression<'a>>, Box<MathExpression<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct Math<'a> {
    pub content: Vec<MathExpression<'a>>,
}