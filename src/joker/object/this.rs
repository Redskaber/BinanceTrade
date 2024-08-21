//! This file is type.rs
//!
//!
//!   #[derive(Debug, Clone, PartialEq)]
//! - Object
//!     - Literal
//!     - Caller
//!
//!
//!   #[derive(Debug, Clone, PartialEq)]
//! - Literal
//!     - I32
//!     - F64
//!     - Str
//!     - Bool
//!     - Null
//!
//!   #[derive(Debug, Clone, PartialEq)]
//! - Caller: impl Callable
//!     - Function
//!
//!
//! - Function
//!     - NativeFunction
//!     - UserFunction
//!
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use super::{Caller, Instance, Literal};

pub trait UpCast<T> {
    fn upcast(&self) -> T;
    fn upcast_into(self) -> T;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Object {
    Literal(Literal),
    Caller(Caller),
    Instance(Instance),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Literal(literal) => Display::fmt(literal, f),
            Object::Caller(caller) => Display::fmt(caller, f),
            Object::Instance(instance) => Display::fmt(instance, f),
        }
    }
}
