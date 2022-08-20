use std::collections::HashMap;
use super::tokens::Token;

pub enum Path {
    ExpressionHead,
    Identifier(Token),
    Direct{storage_type: String, source: String, path: String},
    Member(Box<Path>, Token),
    Element(Box<Path>, Token),
    Quary(Box<Path>, HashMap<String, ConstExpression>)
}

pub enum RefExpression {
    Path(Path),
    BinaryAssign(Box<RefExpression>, Token, Box<Expression>),
    PostUniOperation(Box<RefExpression>, Token),
    PreUniOperation(Token, Box<RefExpression>),
    Indexing(Box<Expression>, ConstExpression),
    InnerValue(Box<Expression>, Token),
    Cast(Path, Box<Expression>)
}

pub enum ConstExpression {
    Literal(Token),
    Object(HashMap<Token, ConstExpression>),
    List(Vec<ConstExpression>),
    BinaryOperation(Box<ConstExpression>, Token, Box<ConstExpression>),
    Tuple(Box<ConstExpression>)
}

pub enum Expression {
    Ref(RefExpression),
    Object(HashMap<Token, Expression>),
    List(Vec<Expression>),
    Const(ConstExpression),
    Tuple(Box<Expression>),
    Call(RefExpression, Vec<Expression>),
    BinaryOperation(Box<Expression>, Token, Box<Expression>),
    Swap(RefExpression, RefExpression)
}