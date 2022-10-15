use crate::ast::*;
use std::cmp::max;
use std::collections::HashMap;

/// Find the maximum number of expression in a print statement
fn maxargs(statement: &Statement) -> usize {
    // we move Statement into this function instead of borrowing it, we can probably change this
    match statement {
        Statement::Compound(l, r) => max(maxargs(l), maxargs(r)),
        Statement::Assign(_, expr) => maxargs_expr(expr),
        Statement::Print(exprs) => max(
            exprs.len(),
            exprs
                .into_iter()
                .map(|e| maxargs_expr(e))
                .max()
                .unwrap_or(0),
        ),
    }
}

fn maxargs_expr(expr: &Expression) -> usize {
    match expr {
        Expression::Identifier(_) => 0,
        Expression::Num(_) => 0,
        Expression::Op(l, _, r) => max(maxargs_expr(l), maxargs_expr(r)),
        Expression::Eseq(statement, expr) => max(maxargs(statement), maxargs_expr(expr)),
    }
}

fn interp_statement(
    statement: &Statement,
    symbols: &mut HashMap<String, i64>,
    output: &mut Vec<String>,
) {
    match statement {
        Statement::Compound(l, r) => {
            interp_statement(l, symbols, output);
            interp_statement(r, symbols, output);
        }
        Statement::Assign(identifier, expr) => {
            let res = interp_expr(expr, symbols, output);
            symbols.insert(identifier.clone(), res);
        }
        Statement::Print(exprs) => {
            let res = exprs
                .iter()
                .map(|e| interp_expr(&e, symbols, output))
                .map(|n| n.to_string())
                .collect::<String>();

            output.push(res);
        }
    }
}

fn interp_expr(
    expr: &Expression,
    symbols: &mut HashMap<String, i64>,
    output: &mut Vec<String>,
) -> i64 {
    match expr {
        Expression::Identifier(identifier) => *symbols.get(identifier).unwrap(),
        Expression::Num(n) => *n,
        Expression::Op(l, op, r) => {
            let left_value = interp_expr(l, symbols, output);
            let right_value = interp_expr(l, symbols, output);
            match op {
                BinaryOp::Plus => left_value + right_value,
                BinaryOp::Minus => left_value - right_value,
                BinaryOp::Times => left_value * right_value,
                BinaryOp::Div => left_value / right_value,
            }
        }
        Expression::Eseq(statement, expr) => {
            interp_statement(statement, symbols, output);
            interp_expr(expr, symbols, output)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // const expressions aren't a thing? so I will do this instead
    // makes sense to not use a const expr since I have all these heap
    // allocations! (although I guess you could jam this in a text segment?)
    fn prog() -> Statement {
        use Expression::*;
        use Statement::*;

        // Example straight line program:
        // ```
        // a := 5 + 3
        // b := (print(a, a - 1), 10*a)
        // print(b)
        // ```
        Compound(
            Box::new(Assign(
                "a".to_string(),
                Box::new(Op(Box::new(Num(5)), BinaryOp::Plus, Box::new(Num(3)))),
            )),
            Box::new(Compound(
                Box::new(Assign(
                    "b".to_string(),
                    Box::new(Eseq(
                        Box::new(Print(vec![
                            Box::new(Identifier("a".to_string())),
                            Box::new(Op(
                                Box::new(Identifier("a".to_string())),
                                BinaryOp::Minus,
                                Box::new(Num(1)),
                            )),
                        ])),
                        Box::new(Op(
                            Box::new(Num(10)),
                            BinaryOp::Times,
                            Box::new(Identifier("a".to_string())),
                        )),
                    )),
                )),
                Box::new(Print(vec![Box::new(Identifier("b".to_string()))])),
            )),
        )
    }

    #[test]
    fn maxargs_on_prog() {
        assert_eq!(maxargs(&prog()), 2)
    }

    #[test]
    fn interp_on_prog() {
        let mut output = Vec::new();
        interp_statement(&prog(), &mut HashMap::new(), &mut output);
        assert_eq!(output, vec!["8 7", "80"])
    }
}
