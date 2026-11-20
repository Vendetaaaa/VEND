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
///
///
/// Parses, validates, lowers, and emits a complete VEND source program.
///
pub fn compile(source: &str, abi: Abi) -> Result<String, Vec<Diagnostic>> {
    let program = parse(source)?;

    let mut diagnostics = analyze_program(&program);
    for function in &program.functions {
        diagnostics.extend(validate_abi(function, abi));
    }
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    emit_program(&lower_program(&program)).map_err(|error| vec![Diagnostic::new(error.to_string())])
}
