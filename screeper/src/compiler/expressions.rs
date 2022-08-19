use super::tokens::Token;

pub enum Path {
    ExpressionHead,
    Identifier(Token),
    Member(Box<Path>, Token)
}

pub enum RefExpression {
    Path(Path),
    Assign(Box<RefExpression>, Box<Expression>),
    AddAssign(Box<RefExpression>, Box<Expression>),
    SubtractAssign(Box<RefExpression>, Box<Expression>),
    MultiplyAssign(Box<RefExpression>, Box<Expression>),
    DivideAssign(Box<RefExpression>, Box<Expression>),
    ModulusAssign(Box<RefExpression>, Box<Expression>),
    PreIncrement(Box<RefExpression>),
    PostIncrement(Box<RefExpression>),
    PreDecrement(Box<RefExpression>),
    PostDecrement(Box<RefExpression>),
    Indexing(Box<Expression>, Box<ConstExpression>),
    InnerValue(Box<Expression>, Token),
    Cast(Path, Box<Expression>)
}

pub enum ConstExpression {
    
}

pub enum Expression {

}