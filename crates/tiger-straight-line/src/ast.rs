type Id = String;

pub enum BinaryOp {
  Plus,
  Minus,
  Times,
  Div
}

pub enum Statement {
  Compound(Box<Statement>, Box<Statement>),
  Assign(Id, Box<Expression>),
  Print(Vec<Box<Expression>>)
}

pub enum Expression {
  Identifier(Id),
  Num(i64),
  Op(Box<Expression>, BinaryOp, Box<Expression>),
  Eseq(Box<Statement>, Box<Expression>)
}