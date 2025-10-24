use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub enum TypeErrorKind {
    /// left_type, right_type, op
    UnsupportedOperandType(Rc<str>, Rc<str>, Rc<str>),
}

#[derive(Debug, Clone)]
pub struct TypeError {
    error_kind: TypeErrorKind,
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = match &self.error_kind {
            TypeErrorKind::UnsupportedOperandType(l_ty, r_ty, op) => {
                format!("unsupported operand type(s) for {op}: '{l_ty}' and '{r_ty}'")
            }
        };

        write!(f, "type error: {info}")
    }
}

impl From<TypeErrorKind> for TypeError {
    fn from(value: TypeErrorKind) -> Self {
        Self {
            error_kind: value
        }
    }
}