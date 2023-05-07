//! AST pretty-printer
use std::cell::RefCell;
use std::fmt::{Display, Formatter};

use crate::expr::{Expr, MemberAccess, MemberPath, Operator, OperatorKind};
use crate::ident::Ident;

#[allow(dead_code)]
#[rustfmt::skip]
mod color {
    pub(super) const FG_BLACK:   &str = "\x1b[30m";
    pub(super) const FG_RED:     &str = "\x1b[31m";
    pub(super) const FG_GREEN:   &str = "\x1b[32m";
    pub(super) const FG_YELLOW:  &str = "\x1b[33m";
    pub(super) const FG_BLUE:    &str = "\x1b[34m";
    pub(super) const FG_MAGENTA: &str = "\x1b[35m";
    pub(super) const FG_CYAN:    &str = "\x1b[36m";
    pub(super) const FG_WHITE:   &str = "\x1b[37m";
}

pub struct PrettyExpr<'a> {
    expr: &'a Expr,
    depth: RefCell<String>,
}

impl<'a> PrettyExpr<'a> {
    pub fn new(expr: &'a Expr) -> Self {
        Self {
            expr,
            depth: RefCell::new(String::with_capacity(0xFF)),
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
        use color::*;
        write!(f, "{FG_GREEN}{}{FG_WHITE}", self.depth.borrow())
    }

    fn write_colour(&self, f: &mut Formatter, text: &str, color: &str) -> std::fmt::Result {
        use color::*;
        write!(f, "{color}{text}{FG_WHITE}")
    }

    fn fmt_expr(&self, f: &mut Formatter, expr: &Expr) -> std::fmt::Result {
        use color::*;

        match expr {
            Expr::Unknown => writeln!(f, "unknown")?,
            Expr::Unary(unary) => {
                // op
                self.fmt_operator(f, &unary.operator)?;

                // rhs
                self.fmt_prefix(f)?;
                self.write_colour(f, "└─", color::FG_GREEN)?;
                self.push_prefix("  ");
                self.fmt_expr(f, &unary.rhs)?;
                self.pop_prefix(2);
            }
            Expr::Binary(binary) => {
                // op
                self.fmt_operator(f, &binary.operator)?;

                // lhs
                self.fmt_prefix(f)?;
                self.write_colour(f, "├─", color::FG_GREEN)?;
                self.push_prefix("| ");
                self.fmt_expr(f, &binary.lhs)?;
                self.pop_prefix(2);

                // rhs
                self.fmt_prefix(f)?;
                self.write_colour(f, "└─", color::FG_GREEN)?;
                self.push_prefix("  ");
                self.fmt_expr(f, &binary.rhs)?;
                self.pop_prefix(2);
            }
            Expr::Assign(assign) => {
                // op
                writeln!(f, "assign")?;

                // lhs
                self.fmt_prefix(f)?;
                self.write_colour(f, "├─", color::FG_GREEN)?;
                self.fmt_ident(f, &assign.lhs)?;

                // rhs
                self.fmt_prefix(f)?;
                self.write_colour(f, "└─", color::FG_GREEN)?;
                self.push_prefix("  ");
                self.fmt_expr(f, &assign.rhs)?;
                self.pop_prefix(2);
            }
            Expr::Num(num) => writeln!(f, "number {FG_MAGENTA}\"{}\"{FG_WHITE}", num.value)?,
            Expr::Group(group) => {
                writeln!(f, "group")?;

                self.fmt_prefix(f)?;
                self.write_colour(f, "└─", color::FG_GREEN)?;
                self.push_prefix("  ");
                self.fmt_expr(f, &group.expr)?;
                self.pop_prefix(2);
            }
            Expr::NameAccess(access) => {
                writeln!(f, "name_access {FG_MAGENTA}\"{}\"{FG_WHITE}", access.ident.text)?;
            }
            Expr::MemberAccess(access) => {
                self.fmt_member_access(f, access)?;
            }
            Expr::MemberAssign(assign) => {
                writeln!(f, "member_assign")?;

                // lhs
                self.fmt_prefix(f)?;
                self.write_colour(f, "├─", color::FG_GREEN)?;
                self.push_prefix("| ");
                self.fmt_member_path(f, &assign.path)?;
                self.pop_prefix(2);

                self.fmt_prefix(f)?;
                self.write_colour(f, "├─", color::FG_GREEN)?;
                self.fmt_ident(f, &assign.name)?;

                // rhs
                self.fmt_prefix(f)?;
                self.write_colour(f, "└─", color::FG_GREEN)?;
                self.push_prefix("  ");
                self.fmt_expr(f, &assign.rhs)?;
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

    fn fmt_ident(&self, f: &mut Formatter, ident: &Ident) -> std::fmt::Result {
        use color::*;
        writeln!(f, "ident {FG_MAGENTA}\"{}\"{FG_WHITE}", ident.text)
    }

    fn fmt_member_access(&self, f: &mut Formatter, access: &MemberAccess) -> std::fmt::Result {
        writeln!(f, "member_access")?;

        // path
        self.fmt_prefix(f)?;
        self.write_colour(f, "├─", color::FG_GREEN)?;
        self.push_prefix("| ");
        self.fmt_member_path(f, &access.path)?;
        self.pop_prefix(2);

        // name
        self.fmt_prefix(f)?;
        self.write_colour(f, "└─", color::FG_GREEN)?;
        self.push_prefix("  ");
        self.fmt_ident(f, &access.name)?;
        self.pop_prefix(2);

        Ok(())
    }

    fn fmt_member_path(&self, f: &mut Formatter, path: &MemberPath) -> std::fmt::Result {
        writeln!(f, "member_path")?;

        match path {
            MemberPath::Name(ident) => {
                self.fmt_prefix(f)?;
                self.write_colour(f, "└─", color::FG_GREEN)?;
                self.push_prefix("| ");
                self.fmt_ident(f, ident)?;
                self.pop_prefix(2);
            }
            MemberPath::Path(access) => {
                self.fmt_prefix(f)?;
                self.write_colour(f, "└─", color::FG_GREEN)?;
                self.push_prefix("  ");
                self.fmt_member_access(f, access)?;
                self.pop_prefix(2);
            }
        }

        Ok(())
    }
}

impl<'a> Display for PrettyExpr<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.fmt_expr(f, &self.expr)
    }
}
