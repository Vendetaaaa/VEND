use vend_ast::{
    AliasDecl, ComparisonOp, Condition, FunctionDecl, If, IndexedRegister, Instruction,
    MemoryAddress, Mnemonic, Operand, PhysicalRegister, Program, Register, RegisterWidth,
    ScopeBlock, Statement, While,
};

#[test]
fn builds_nested_ast_without_parser_state() {
    let alias = AliasDecl {
        name: "ptr".into(),
        width: RegisterWidth::Reg64,
        physical: PhysicalRegister::RAX,
        span: None,
    };

    let load = Instruction {
        mnemonic: Mnemonic("mov".into()),
        operands: vec![
            Operand::Register(Register::Alias("ptr".into())),
            Operand::MemoryAddress(MemoryAddress {
                base: Some(Register::Physical(PhysicalRegister::RBP)),
                index: Some(IndexedRegister {
                    register: Register::Physical(PhysicalRegister::R8),
                    scale: 4,
                }),
                displacement: 16,
            }),
        ],
    };

    let condition = Condition {
        left: Operand::Register(Register::Physical(PhysicalRegister::RAX)),
        op: ComparisonOp::Gt,
        right: Operand::Immediate(0),
    };

    let loop_block = ScopeBlock {
        statements: vec![Statement::Instruction(Instruction {
            mnemonic: Mnemonic("add".into()),
            operands: vec![
                Operand::Register(Register::Alias("ptr".into())),
                Operand::Immediate(8),
            ],
        })],
    };

    let program = Program {
        functions: vec![FunctionDecl {
            name: "main".into(),
            params: vec!["argc".into(), "argv".into()],
            body: ScopeBlock {
                statements: vec![
                    Statement::AliasDecl(alias),
                    Statement::Instruction(load),
                    Statement::If(If {
                        condition: condition.clone(),
                        then_block: ScopeBlock {
                            statements: vec![Statement::While(While {
                                condition,
                                body: loop_block,
                            })],
                        },
                        else_block: Some(ScopeBlock {
                            statements: vec![Statement::Instruction(Instruction {
                                mnemonic: Mnemonic("xor".into()),
                                operands: vec![
                                    Operand::Register(Register::Physical(PhysicalRegister::RAX)),
                                    Operand::Register(Register::Physical(PhysicalRegister::RAX)),
                                ],
                            })],
                        }),
                    }),
                ],
            },
        }],
    };

    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].name, "main");
    assert_eq!(RegisterWidth::Reg64.bits(), 64);
    assert_eq!(PhysicalRegister::R8B.width(), RegisterWidth::Reg8);
    assert_eq!(PhysicalRegister::RAX.as_str(), "RAX");
}

#[test]
fn register_width_names_match_textual_form() {
    assert_eq!(RegisterWidth::Reg64.as_str(), "reg64");
    assert_eq!(RegisterWidth::Reg32.as_str(), "reg32");
    assert_eq!(RegisterWidth::Reg16.as_str(), "reg16");
    assert_eq!(RegisterWidth::Reg8.as_str(), "reg8");
}

#[test]
fn physical_register_overlap_is_detected() {
    // 64 vs 32
    assert!(PhysicalRegister::RAX.overlaps(PhysicalRegister::EAX));
    // 16 vs L/H 8
    assert!(PhysicalRegister::AX.overlaps(PhysicalRegister::AH));
    assert!(PhysicalRegister::AX.overlaps(PhysicalRegister::AL));
    // AH and AL
    assert!(!PhysicalRegister::AH.overlaps(PhysicalRegister::AL));
    // roots dont overlap
    assert!(!PhysicalRegister::RAX.overlaps(PhysicalRegister::RBX));
    // R8
    assert!(PhysicalRegister::R8.overlaps(PhysicalRegister::R8D));
    assert!(PhysicalRegister::R8W.overlaps(PhysicalRegister::R8B));
}

#[test]
fn every_physical_register_has_expected_root_and_range() {
    let groups = [
        (0, (0, 64), [PhysicalRegister::RAX, PhysicalRegister::EAX, PhysicalRegister::AX, PhysicalRegister::AL, PhysicalRegister::AH]),
        (1, (0, 64), [PhysicalRegister::RBX, PhysicalRegister::EBX, PhysicalRegister::BX, PhysicalRegister::BL, PhysicalRegister::BH]),
        (2, (0, 64), [PhysicalRegister::RCX, PhysicalRegister::ECX, PhysicalRegister::CX, PhysicalRegister::CL, PhysicalRegister::CH]),
        (3, (0, 64), [PhysicalRegister::RDX, PhysicalRegister::EDX, PhysicalRegister::DX, PhysicalRegister::DL, PhysicalRegister::DH]),
    ];

    for (root, _, registers) in groups {
        for register in registers {
            assert_eq!(register.root_index(), root);
        }
    }

    assert_eq!(PhysicalRegister::RAX.bit_range(), (0, 64));
    assert_eq!(PhysicalRegister::EAX.bit_range(), (0, 32));
    assert_eq!(PhysicalRegister::AX.bit_range(), (0, 16));
    assert_eq!(PhysicalRegister::AL.bit_range(), (0, 8));
    assert_eq!(PhysicalRegister::AH.bit_range(), (8, 8));
    for register in [
        PhysicalRegister::RSI, PhysicalRegister::RDI, PhysicalRegister::RBP,
        PhysicalRegister::RSP, PhysicalRegister::R8, PhysicalRegister::R9,
        PhysicalRegister::R10, PhysicalRegister::R11, PhysicalRegister::R12,
        PhysicalRegister::R13, PhysicalRegister::R14, PhysicalRegister::R15,
    ] {
        assert_eq!(register.bit_range(), (0, 64));
    }
}

