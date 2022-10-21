use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;

// TODO 
// sketch out bones of grammar
// build out test suite (from Appel's example programs?)
// think about details of grammar, precedence, comments and whitespace etc.
//   refer to other LALRPOP grammars in the wild like: https://github.com/RustPython/RustPython/blob/main/compiler/parser/python.lalrpop

#[cfg(test)]
mod tests {
  use crate::grammar;
  use crate::ast;

  #[test]
  fn parse_simple_program() {
    assert_eq!(grammar::ProgramParser::new().parse("2").unwrap(), ast::Program{ expr: ast::Expr::NumberLiteral(2) });
  }


  #[test]
  fn parse_short_examples() {
    let programs = vec![
      "abc",
      "abc.def",
      "abc[123]"
    ];
    for ele in programs {
      let result = grammar::ProgramParser::new().parse(ele);
      if result.is_err() {
        assert!(false, "{} {:?}", ele, result);
      }
    }
  }

  #[test]
  fn parse_testcase() {
    assert!(grammar::ProgramParser::new().parse(include_str!("testcases/test1.tig")).is_ok())
  }
}
