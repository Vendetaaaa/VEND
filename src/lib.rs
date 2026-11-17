//! Core language model for VEND;
//! AST grammar first, then the rest

pub mod ast;
pub mod grammar;
pub mod analysis;
pub mod parser;
pub mod lowering;
pub mod emitter;
pub mod abi;

pub use ast::*;
pub use analysis::*;
pub use lowering::*;
pub use emitter::*;
pub use abi::*;
pub use parser::{parse, Parser, Lexer};
