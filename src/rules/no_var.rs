// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.
use super::{Context, LintRule, DUMMY_NODE};
use crate::ProgramRef;
use deno_ast::swc::ast::VarDecl;
use deno_ast::swc::ast::VarDeclKind;
use deno_ast::swc::visit::noop_visit_type;
use deno_ast::swc::visit::Node;
use deno_ast::swc::visit::Visit;
use std::sync::Arc;

#[derive(Debug)]
pub struct NoVar;

const MESSAGE: &str = "`var` keyword is not allowed.";
const CODE: &str = "no-var";

impl LintRule for NoVar {
  fn new() -> Arc<Self> {
    Arc::new(NoVar)
  }

  fn tags(&self) -> &'static [&'static str] {
    &["recommended"]
  }

  fn code(&self) -> &'static str {
    CODE
  }

  fn lint_program<'view>(
    &self,
    context: &mut Context<'view>,
    program: ProgramRef<'view>,
  ) {
    let mut visitor = NoVarVisitor::new(context);
    match program {
      ProgramRef::Module(m) => visitor.visit_module(m, &DUMMY_NODE),
      ProgramRef::Script(s) => visitor.visit_script(s, &DUMMY_NODE),
    }
  }

  #[cfg(feature = "docs")]
  fn docs(&self) -> &'static str {
    include_str!("../../docs/rules/no_var.md")
  }
}

struct NoVarVisitor<'c, 'view> {
  context: &'c mut Context<'view>,
}

impl<'c, 'view> NoVarVisitor<'c, 'view> {
  fn new(context: &'c mut Context<'view>) -> Self {
    Self { context }
  }
}

impl<'c, 'view> Visit for NoVarVisitor<'c, 'view> {
  noop_visit_type!();

  fn visit_var_decl(&mut self, var_decl: &VarDecl, _parent: &dyn Node) {
    if var_decl.kind == VarDeclKind::Var {
      self.context.add_diagnostic(var_decl.span, CODE, MESSAGE);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no_var_valid() {
    assert_lint_ok!(NoVar, r#"let foo = 0; const bar = 1"#,);
  }

  #[test]
  fn no_var_invalid() {
    assert_lint_err!(
      NoVar,
      "var foo = 0;": [{
        col: 0,
        message: MESSAGE,
      }],
      "let foo = 0; var bar = 1;": [{
        col: 13,
        message: MESSAGE,
      }],
      "let foo = 0; var bar = 1; var x = 2;": [
        {
          col: 13,
          message: MESSAGE,
        },
        {
          col: 26,
          message: MESSAGE,
        }
      ]
    );
  }
}
