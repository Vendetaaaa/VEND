use vend_ast::{emit_function, lower_function, parse};

#[test]
fn emits_aliases_memory_and_control_flow_in_intel_syntax() {
    let program = parse(
        "fn load() {
            alias ptr: reg64 = RBP
            alias index: reg64 = R8
            while ptr > 0 {
                mov RAX, [ptr + index * 4 + 16]
            }
        }",
    )
    .expect("source should parse");

    let assembly = emit_function(&lower_function(&program.functions[0]))
        .expect("lowered function should emit");

    assert!(assembly.contains(".intel_syntax noprefix"));
    assert!(assembly.contains("mov RAX, [RBP + R8*4 + 16]"));
    assert!(assembly.contains("cmp RBP, 0"));
    assert!(assembly.contains("jg __vend_while_body_1"));
    assert!(assembly.contains("jmp __vend_while_head_0"));
}

#[test]
fn emits_all_comparison_jump_forms() {
    let program = parse(
        "fn compare() {
            if RAX == RBX { nop }
            if RAX != RBX { nop }
            if RAX < RBX { nop }
            if RAX <= RBX { nop }
            if RAX > RBX { nop }
            if RAX >= RBX { nop }
        }",
    )
    .expect("source should parse");
    let assembly = emit_function(&lower_function(&program.functions[0]))
        .expect("lowered function should emit");

    for mnemonic in ["je ", "jne ", "jl ", "jle ", "jg ", "jge "] {
        assert!(assembly.contains(mnemonic), "missing {mnemonic}");
    }
}

/// Hmmmmm
