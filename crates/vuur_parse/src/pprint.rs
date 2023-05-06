//! AST pretty-printer
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::fmt::{Display, Formatter};

use colored::*;

use crate::expr::{Expr, Operator, OperatorKind};

pub struct PrettyExpr<'a> {
    expr: &'a Expr,
    depth: RefCell<String>,
}

impl<'a> PrettyExpr<'a> {
    pub fn new(expr: &'a Expr) -> Self {
        Self {
            expr,
            depth: RefCell::new(String::new()),
        }
    }

    fn push_prefix(&self, prefix: impl AsRef<str>) {
        self.depth.borrow_mut().push_str(prefix.as_ref())
    }

    fn pop_prefix(&self, char_count: usize) {
        let mut depth = self.depth.borrow_mut();
        for _ in 0..char_count {
            depth.pop();
        }
    }

    fn fmt_prefix(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.depth.borrow().green())
    }

    fn fmt_expr(&self, f: &mut Formatter, expr: &Expr) -> std::fmt::Result {
        match expr {
            Expr::Num(num) => writeln!(f, "number \"{}\"", num.value)?,
            Expr::Unary(unary) => {
                // op
                self.fmt_operator(f, &unary.operator)?;

                // rhs
                self.fmt_prefix(f)?;
                write!(f, "{}", "└─".green())?;
                self.push_prefix("  ");
                self.fmt_expr(f, &unary.rhs)?;
                self.pop_prefix(2);
            }
            Expr::Binary(binary) => {
                // op
                self.fmt_operator(f, &binary.operator)?;

                // lhs
                self.fmt_prefix(f)?;
                write!(f, "{}", "├─".green())?;
                self.push_prefix("| ");
                self.fmt_expr(f, &binary.lhs)?;
                self.pop_prefix(2);

                // rhs
                self.fmt_prefix(f)?;
                write!(f, "{}", "└─".green())?;
                self.push_prefix("  ");
                self.fmt_expr(f, &binary.rhs)?;
                self.pop_prefix(2);
            }
            Expr::Group(group) => {
                writeln!(f, "group")?;

                self.fmt_prefix(f)?;
                write!(f, "{}", "└─".green())?;
                self.push_prefix("  ");
                self.fmt_expr(f, &group.expr)?;
                self.pop_prefix(2);
            }
            _ => todo!("{expr:?}"),
        }

        Ok(())
    }

    fn fmt_operator(&self, f: &mut Formatter, op: &Operator) -> std::fmt::Result {
        match op.kind {
            OperatorKind::Neg => writeln!(f, "negate"),
            OperatorKind::Add => writeln!(f, "add"),
            OperatorKind::Sub => writeln!(f, "subtract"),
            OperatorKind::Mul => writeln!(f, "multiply"),
            OperatorKind::Div => writeln!(f, "divide"),
            OperatorKind::Assign => writeln!(f, "assign"),
            OperatorKind::Equals => writeln!(f, "equals"),
        }
    }
}

impl<'a> Display for PrettyExpr<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.fmt_expr(f, &self.expr)
    }
}
