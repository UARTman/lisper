use std::fmt::Display;

use crate::parser::ast::ASTNode;

use super::{native_functions::NativeFn, native_macros::NativeMacro};

#[derive(Clone)]
pub enum Data {
    Integer(i64),
    String(String),
    Boolean(bool),
    NativeFunction(Box<NativeFn>),
    NativeMacro(Box<NativeMacro>),
    Quote(Box<ASTNode>),
    Lambda(Vec<String>, ASTNode),
    Null,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::NativeFunction(_), Self::NativeFunction(_)) => false,
            (Self::NativeMacro(_), Self::NativeMacro(_)) => false,
            (Self::Quote(l0), Self::Quote(r0)) => l0 == r0,
            (Self::Lambda(l0, l1), Self::Lambda(r0, r1)) => l0 == r0 && l1 == r1,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Data {
    pub fn as_integer(&self) -> Option<i64> {
        if let Data::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn as_string(&self) -> Option<String> {
        if let Data::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Data::Boolean(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn as_native_function(&self) -> Option<&NativeFn> {
        if let Data::NativeFunction(f) = self {
            Some(f)
        } else {
            None
        }
    }
    pub fn as_native_macro(&self) -> Option<&NativeMacro> {
        if let Data::NativeMacro(f) = self {
            Some(f)
        } else {
            None
        }
    }
    pub fn as_quote(&self) -> Option<&ASTNode> {
        if let Data::Quote(f) = self {
            Some(&**f)
        } else {
            None
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Integer(i) => i.fmt(f),
            Data::String(s) => s.fmt(f),
            Data::NativeFunction(_) => f.write_fmt(format_args!("<Native function>")),
            Data::NativeMacro(_) => f.write_fmt(format_args!("<Native macro>")),
            Data::Null => f.write_fmt(format_args!("<Null>")),
            Data::Boolean(b) => b.fmt(f),
            Data::Quote(q) => f.write_fmt(format_args!("<Quote {q:?}>")),
            Data::Lambda(_, _) => f.write_fmt(format_args!("<Function>")),
        }
    }
}
