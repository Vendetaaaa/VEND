use vend_ast::{
    lower_function, parse, ComparisonOp, LoweredInstruction, Operand, PhysicalRegister, Register,
};

#[test]
fn lowers_if_with_else_to_deterministic_branches() {
    let program = parse("fn choose() { if RAX > 0 { mov RBX 1 } else { mov RBX 0 } }")
        .expect("source should parse");

    let lowered = lower_function(&program.functions[0]);

    assert_eq!(
        lowered.instructions,
        vec![
            LoweredInstruction::Compare {
                left: Operand::Register(Register::Physical(PhysicalRegister::RAX)),
                right: Operand::Immediate(0),
            },
            LoweredInstruction::Branch {
                condition: ComparisonOp::Gt,
                target: "__vend_if_then_0".into(),
            },
            LoweredInstruction::Jump {
                target: "__vend_if_else_2".into(),
            },
            LoweredInstruction::Label("__vend_if_then_0".into()),
            LoweredInstruction::Instruction(vend_ast::Instruction {
                mnemonic: vend_ast::Mnemonic("mov".into()),
                operands: vec![
                    Operand::Register(Register::Physical(PhysicalRegister::RBX)),
                    Operand::Immediate(1),
                ],
            }),
            LoweredInstruction::Jump {
                target: "__vend_if_end_1".into(),
            },
            LoweredInstruction::Label("__vend_if_else_2".into()),
            LoweredInstruction::Instruction(vend_ast::Instruction {
                mnemonic: vend_ast::Mnemonic("mov".into()),
                operands: vec![
                    Operand::Register(Register::Physical(PhysicalRegister::RBX)),
                    Operand::Immediate(0),
                ],
            }),
            LoweredInstruction::Label("__vend_if_end_1".into()),
        ]
    );
}

#[test]
fn lowers_while_with_back_edge_and_omits_alias_declarations() {
    let program = parse("fn loop() { alias count: reg64 = RCX while count > 0 { add count 1 } }")
        .expect("source should parse");

    let lowered = lower_function(&program.functions[0]);

    assert!(matches!(
        lowered.instructions.first(),
        Some(LoweredInstruction::Label(label)) if label == "__vend_while_head_0"
    ));
    assert!(lowered.instructions.iter().any(|instruction| {
        matches!(
            instruction,
            LoweredInstruction::Jump { target } if target == "__vend_while_head_0"
        )
    }));
    assert_eq!(
        lowered
            .instructions
            .iter()
            .filter(|instruction| matches!(instruction, LoweredInstruction::Instruction(_)))
            .count(),
        1
    );
}
