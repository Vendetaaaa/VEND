use vend_ast::{
    parse, ComparisonOp, Operand, PhysicalRegister, Register, RegisterWidth, Statement,
};

#[test]
fn parser_empty_program() {
    let input = "";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    assert_eq!(prog.functions.len(), 0);
}

#[test]
fn parser_simple_function_no_params() {
    let input = "fn main() { }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    assert_eq!(prog.functions.len(), 1);
    assert_eq!(prog.functions[0].name, "main");
    assert_eq!(prog.functions[0].params.len(), 0);
}

#[test]
fn parser_function_with_params() {
    let input = "fn add(a, b, c) { }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    assert_eq!(prog.functions[0].name, "add");
    assert_eq!(prog.functions[0].params, vec!["a", "b", "c"]);
}

#[test]
fn parser_alias_declaration() {
    let input = "fn test() { alias x : reg64 = RAX }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    assert_eq!(block.statements.len(), 1);
    if let Statement::AliasDecl(a) = &block.statements[0] {
        assert_eq!(a.name, "x");
        assert_eq!(a.width, RegisterWidth::Reg64);
        assert_eq!(a.physical, PhysicalRegister::RAX);
    } else {
        panic!("Expected alias declaration");
    }
}

#[test]
fn parser_multiple_alias_declarations() {
    let input = "fn test() { alias a : reg64 = RAX alias b : reg32 = EBX }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    assert_eq!(block.statements.len(), 2);
}

#[test]
fn parser_instruction_no_operands() {
    let input = "fn test() { nop }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        assert_eq!(instr.mnemonic.0, "nop");
        assert_eq!(instr.operands.len(), 0);
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_instruction_one_operand() {
    let input = "fn test() { push RAX }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        assert_eq!(instr.operands.len(), 1);
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_instruction_two_operands() {
    let input = "fn test() { mov RAX RBX }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        assert_eq!(instr.operands.len(), 2);
        if let Operand::Register(Register::Physical(r1)) = &instr.operands[0] {
            assert_eq!(*r1, PhysicalRegister::RAX);
        } else {
            panic!("Expected physical register");
        }
        if let Operand::Register(Register::Physical(r2)) = &instr.operands[1] {
            assert_eq!(*r2, PhysicalRegister::RBX);
        } else {
            panic!("Expected physical register");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_instruction_with_immediate() {
    let input = "fn test() { mov RAX 42 }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        if let Operand::Immediate(i) = &instr.operands[1] {
            assert_eq!(*i, 42);
        } else {
            panic!("Expected immediate");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_instruction_with_negative_immediate() {
    let input = "fn test() { mov RAX -10 }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        if let Operand::Immediate(i) = &instr.operands[1] {
            assert_eq!(*i, -10);
        } else {
            panic!("Expected immediate");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_instruction_with_alias_register() {
    let input = "fn test() { mov myvar 100 }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        if let Operand::Register(Register::Alias(a)) = &instr.operands[0] {
            assert_eq!(a, "myvar");
        } else {
            panic!("Expected alias register");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_memory_operand_base_only() {
    let input = "fn test() { mov RAX [RBX] }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        if let Operand::MemoryAddress(addr) = &instr.operands[1] {
            assert!(addr.base.is_some());
            assert!(addr.index.is_none());
            assert_eq!(addr.displacement, 0);
        } else {
            panic!("Expected memory address");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_memory_operand_with_index_and_scale() {
    let input = "fn test() { mov RAX [RBX + R8 * 4] }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        if let Operand::MemoryAddress(addr) = &instr.operands[1] {
            assert!(addr.base.is_some());
            assert!(addr.index.is_some());
            if let Some(idx) = &addr.index {
                assert_eq!(idx.scale, 4);
            }
        } else {
            panic!("Expected memory address");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_memory_operand_with_displacement() {
    let input = "fn test() { mov RAX [RBP + 16] }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::Instruction(instr) = &block.statements[0] {
        if let Operand::MemoryAddress(addr) = &instr.operands[1] {
            assert_eq!(addr.displacement, 16);
        } else {
            panic!("Expected memory address");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn parser_if_statement_no_else() {
    let input = "fn test() { if RAX == 0 { } }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::If(if_stmt) = &block.statements[0] {
        assert!(if_stmt.else_block.is_none());
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn parser_if_statement_with_else() {
    let input = "fn test() { if RAX == 0 { } else { } }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::If(if_stmt) = &block.statements[0] {
        assert!(if_stmt.else_block.is_some());
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn parser_comparison_operators() {
    let test_cases = vec![
        ("if RAX == 0 { } ", ComparisonOp::Eq),
        ("if RAX != 0 { } ", ComparisonOp::Ne),
        ("if RAX < 0 { } ", ComparisonOp::Lt),
        ("if RAX > 0 { } ", ComparisonOp::Gt),
        ("if RAX <= 0 { } ", ComparisonOp::Le),
        ("if RAX >= 0 { } ", ComparisonOp::Ge),
    ];

    for (input, expected_op) in test_cases {
        let full_input = format!("fn test() {{ {} }}", input);
        let result = parse(&full_input);
        assert!(result.is_ok(), "Failed to parse: {}", full_input);
        let prog = result.unwrap();
        let block = &prog.functions[0].body;
        if let Statement::If(if_stmt) = &block.statements[0] {
            assert_eq!(if_stmt.condition.op, expected_op);
        } else {
            panic!("Expected if statement");
        }
    }
}

#[test]
fn parser_while_loop() {
    let input = "fn test() { while RAX > 0 { } }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::While(_) = &block.statements[0] {
    } else {
        panic!("Expected while statement");
    }
}

#[test]
fn parser_nested_scope_block() {
    let input = "fn test() { { } }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::ScopeBlock(_) = &block.statements[0] {
    } else {
        panic!("Expected nested scope block");
    }
}

#[test]
fn parser_multiple_functions() {
    let input = "fn f1() { } fn f2() { } fn f3() { }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    assert_eq!(prog.functions.len(), 3);
    assert_eq!(prog.functions[0].name, "f1");
    assert_eq!(prog.functions[1].name, "f2");
    assert_eq!(prog.functions[2].name, "f3");
}

#[test]
fn parser_error_missing_closing_brace() {
    let input = "fn main() {";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(!diags.is_empty());
}

#[test]
fn parser_error_missing_function_name() {
    let input = "fn () { }";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(!diags.is_empty());
}

#[test]
fn parser_error_invalid_token() {
    let input = "fn test() { alias x : reg64 = 123 }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn parser_complex_program() {
    let input = r#"
        fn fibonacci(n) {
            alias counter : reg64 = RAX
            alias result : reg32 = EBX
            
            mov counter 0
            mov result 1
            
            while counter < n {
                mov RDX result
                add result RDX
                add counter 1
            }
            
            if result > 100 {
                mov counter 0
            } else {
                mov counter 1
            }
        }
    "#;
    let result = parse(input);
    assert!(result.is_ok(), "Failed to parse complex program");
    let prog = result.unwrap();
    assert_eq!(prog.functions.len(), 1);
    assert_eq!(prog.functions[0].name, "fibonacci");
}

#[test]
fn parser_preserves_spans() {
    let input = "fn test() { alias x : reg64 = RAX }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::AliasDecl(a) = &block.statements[0] {
        assert!(a.span.is_some());
    } else {
        panic!("Expected alias declaration");
    }
}

#[test]
fn parser_comments() {
    let input = r#"
        fn test() {
            // This is a comment
            mov RAX RBX // Another comment
        }
    "#;
    let result = parse(input);
    assert!(result.is_ok());
}

#[test]
fn parser_condition_with_register_operands() {
    let input = "fn test() { if RAX > RBX { } }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    if let Statement::If(if_stmt) = &block.statements[0] {
        if let Operand::Register(r) = &if_stmt.condition.left {
            assert!(matches!(r, Register::Physical(PhysicalRegister::RAX)));
        } else {
            panic!("Expected physical register");
        }
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn parser_all_register_types() {
    let input = "fn test() { alias a : reg64 = RAX alias b : reg32 = EBX alias c : reg16 = CX alias d : reg8 = DL }";
    let result = parse(input);
    assert!(result.is_ok());
    let prog = result.unwrap();
    let block = &prog.functions[0].body;
    assert_eq!(block.statements.len(), 4);
}

#[test]
fn parser_uses_newline_and_semicolon_as_statement_boundaries() {
    let input = "fn test() {\n mov RAX RBX\n mov RAX, RBX; nop\n }";
    let program = parse(input).expect("separated instructions should parse");
    assert_eq!(program.functions[0].body.statements.len(), 3);
}

#[test]
fn parser_preserves_utf8_byte_offsets_in_diagnostics() {
    let input = "fn tést() { @ }";
    let diagnostics = parse(input).expect_err("invalid character should be diagnosed");
    let at = input.find('@').expect("test marker should exist");
    assert!(diagnostics
        .iter()
        .any(|diagnostic| { diagnostic.span.is_some_and(|span| span.start == at) }));
}
