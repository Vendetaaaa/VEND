use vend_ast::{analyze_liveness, parse, Statement};

#[test]
fn parser_output_preserves_lexical_alias_lifetimes() {
    let program = parse(
        "fn test() { alias outer : reg64 = RAX { mov outer 1 alias inner : reg32 = EBX mov inner 2 } mov inner 3 }",
    )
    .expect("source should parse");

    let report = analyze_liveness(&program.functions[0].body);

    assert_eq!(report.scopes.len(), 2);
    assert_eq!(report.scopes[1].parent, Some(0));
    assert_eq!(report.aliases.len(), 2);
    assert_eq!(report.aliases[0].name, "outer");
    assert_eq!(report.aliases[0].uses, vec![0]);
    assert_eq!(report.aliases[1].name, "inner");
    assert_eq!(report.aliases[1].uses, vec![2]);
    assert_eq!(report.diagnostics.len(), 1);
    assert!(report.diagnostics[0].message.contains("Unknown alias 'inner'"));

    assert!(matches!(program.functions[0].body.statements[1], Statement::ScopeBlock(_)));
}
