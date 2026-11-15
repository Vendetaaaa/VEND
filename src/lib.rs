//! Core language model for VEND;
//! AST grammar first, then the rest

pub mod ast;
pub mod grammar;
pub mod analysis;
pub mod parser;
pub mod lowering;

pub use ast::*;
pub use analysis::*;
pub use lowering::*;
pub use parser::{parse, Parser, Lexer};
