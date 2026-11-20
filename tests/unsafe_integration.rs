use vend_ast::{
    analyze_liveness, analyze_program, emit_function, lower_function, parse, validate_abi, Abi,
    Statement,
};

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

#[test]
fn unsafe_blocks_bypass_operand_intent_checks_but_keep_alias_and_abi_checks() {
    let program = parse(
        "fn main() {
            unsafe {
                alias ptr: reg64 = RAX
                mov RAX, ECX
                mov ptr, [RCX]
                call puts, RAX
            }
        }",
    )
    .expect("unsafe source should parse");

    let diagnostics = analyze_program(&program);
    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.message.contains("matching register widths")),
        "unsafe block should bypass width intent checks"
    );
    assert!(diagnostics
        .iter()
        .all(|diagnostic| { !diagnostic.message.contains("Memory address registers") }));

    let abi_diagnostics = validate_abi(&program.functions[0], Abi::system_v());
    assert_eq!(abi_diagnostics.len(), 1);
    assert!(abi_diagnostics[0].message.contains("ABI call"));

    let liveness = analyze_liveness(&program.functions[0].body);
    assert!(
        liveness.diagnostics.is_empty(),
        "visible aliases should remain lexically traversed"
    );
}

#[test]
fn unsafe_semantics_do_not_disable_unknown_alias_validation() {
    let program =
        parse("fn main() { unsafe { mov missing, RAX } }").expect("unsafe source should parse");

    let diagnostics = analyze_program(&program);
    assert!(diagnostics
        .iter()
        .any(|diagnostic| { diagnostic.message.contains("Unknown alias 'missing'") }));
}

#[test]
fn unsafe_alias_declaration_width_validation_still_applies() {
    let program = parse("fn main() { unsafe { alias bad: reg32 = RAX } }")
        .expect("unsafe source should parse");

    let diagnostics = analyze_program(&program);
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.message.contains("declares reg32")
            && diagnostic
                .message
                .contains("physical register RAX is reg64")
    }));
}

#[test]
fn normal_blocks_keep_operand_intent_checks() {
    let program = parse(
        "fn main() {
            mov RAX, ECX
            mov RAX, [ECX]
        }",
    )
    .expect("normal source should parse");

    let diagnostics = analyze_program(&program);
    assert!(diagnostics
        .iter()
        .any(|diagnostic| { diagnostic.message.contains("matching register widths") }));
    assert!(diagnostics
        .iter()
        .any(|diagnostic| { diagnostic.message.contains("Memory address registers") }));
}
