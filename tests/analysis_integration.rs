use vend_ast::{analyze_program, AliasDecl, FunctionDecl, Program, RegisterWidth, ScopeBlock, Statement, PhysicalRegister};

#[test]
fn program_level_analysis_reports_conflicts() {
    let a1 = AliasDecl { name: "a".into(), width: RegisterWidth::Reg64, physical: PhysicalRegister::RAX, span: None };
    let a2 = AliasDecl { name: "b".into(), width: RegisterWidth::Reg32, physical: PhysicalRegister::EAX, span: None };

    let body = ScopeBlock { statements: vec![Statement::AliasDecl(a1), Statement::AliasDecl(a2)] };

    let prog = Program { functions: vec![FunctionDecl { name: "main".into(), params: vec![], body }] };

    let diags = analyze_program(&prog);
    assert!(!diags.is_empty());
    assert!(diags[0].message.contains("In function 'main'"));
}
