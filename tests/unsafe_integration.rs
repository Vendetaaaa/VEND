use vend_ast::{emit_function, lower_function, parse, Statement};

#[test]
fn parses_and_emits_explicit_unsafe_blocks() {
    let program = parse("fn main() { unsafe { nop } }").expect("unsafe block should parse");
    assert!(matches!(
        program.functions[0].body.statements[0],
        Statement::Unsafe(_)
    ));

    let assembly =
        emit_function(&lower_function(&program.functions[0])).expect("unsafe body should emit");
    assert!(assembly.contains("nop"));
}
