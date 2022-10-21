#[derive(Debug, PartialEq)]
pub struct Program {
  pub expr : Expr
}

#[derive(Debug, PartialEq)]
pub enum LValue {
  Id(String),
  Record(Box<LValue>, String),
  Array(Box<LValue>, Box<Expr>)
}
#[derive(Debug, PartialEq)]
pub enum Expr {
  NumberLiteral(i32),
  LValueExpr(LValue)
}