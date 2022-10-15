use crate::ast::*;
use std::cmp::max;

/// Find the maximum number of expression in a print statement
fn maxargs(statement: Statement) -> usize {
  // we move Statement into this function instead of borrowing it, we can probably change this
  match statement {
    Statement::Compound(l, r) => max(maxargs(*l), maxargs(*r)),
    Statement::Assign(_, expr) => maxargs_expr(*expr),
    Statement::Print(exprs) => max(exprs.len(), exprs.into_iter().map(|e| maxargs_expr(*e)).max().unwrap_or(0)),
  }
}

fn maxargs_expr(expr: Expression) -> usize {
  match expr {
    Expression::Identifier(_) => 0,
    Expression::Num(_) => 0,
    Expression::Op(l, _, r) => max(maxargs_expr(*l), maxargs_expr(*r)),
    Expression::Eseq(statement, expr) => max(maxargs(*statement), maxargs_expr(*expr)),
}
}

#[cfg(test)]
mod tests {
  use super::*;
  // const expressions aren't a thing? so I will do this instead
  // makes sense to not use a const expr since I have all these heap
  // allocations! (although I guess you could jam this in a text segment?)
  fn prog() -> Statement {
    use Statement::*;
    use Expression::*;
    Compound(Box::new(Assign("a".to_string(), Box::new(Op(Box::new(Num(5)), BinaryOp::Plus, Box::new(Num(3)))))),
      Box::new(Compound(Box::new(Assign("b".to_string(),
        Box::new(Eseq(
          Box::new(Print(vec![
            Box::new(Identifier("a".to_string())),
            Box::new(Op(Box::new(Identifier("a".to_string())), BinaryOp::Minus, Box::new(Num(1))))
          ])),
          Box::new(Op(Box::new(Num(10)), BinaryOp::Times, Box::new(Identifier("a".to_string())))))))),
      Box::new(Print(vec![Box::new(Identifier("b".to_string()))]))))
    )
  }

  #[test]
  fn maxargs_on_prog() {
    assert_eq!(maxargs(prog()), 2)
  }
}