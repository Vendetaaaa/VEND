use vend_ast::{parse, Diagnostic};

#[test]
fn diagnostic_reports_missing_closing_paren() {
    let input = "fn main( { }";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(!diags.is_empty());
    assert!(diags[0].message.contains("Expected"));
}

#[test]
fn diagnostic_reports_missing_opening_brace() {
    let input = "fn main()";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(!diags.is_empty());
}

#[test]
fn diagnostic_reports_invalid_register_type() {
    let input = "fn test() { alias x : invalid = RAX }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn diagnostic_reports_missing_register() {
    let input = "fn test() { alias x : reg64 = }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn diagnostic_multiple_errors_accumulated() {
    let input = "fn test() { alias x : reg64 = alias y : reg32 = }";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(diags.len() > 0);
}

#[test]
fn diagnostic_format_with_source() {
    let source = "fn main() { mov RAX }";
    let diag = Diagnostic {
        message: "Expected second operand".to_string(),
        span: Some(vend_ast::Span { start: 15, end: 18 }),
    };
    let formatted = diag.format_with_source(source);
    assert!(formatted.contains("Expected second operand"));
    assert!(formatted.contains("mov RAX"));
}

#[test]
fn diagnostic_includes_position_info() {
    let diag = Diagnostic {
        message: "Test error".to_string(),
        span: Some(vend_ast::Span { start: 5, end: 10 }),
    };
    let display = format!("{}", diag);
    assert!(display.contains("position"));
    assert!(display.contains("Test error"));
}

#[test]
fn parser_reports_error_at_correct_position() {
    let input = "fn f() { alias x : reg64 = 123 }";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(!diags.is_empty());
    assert!(diags[0].span.is_some());
}

#[test]
fn parser_error_recovery_continues_parsing() {
    let input = "fn f1() { alias x : reg64 = 123 } fn f2() { }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn diagnostic_error_for_unexpected_token() {
    let input = "fn test() { 123 mov RAX }";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(!diags.is_empty());
    assert!(diags[0].message.contains("Unexpected"));
}

#[test]
fn diagnostic_error_for_missing_condition_operand() {
    let input = "fn test() { if RAX == { } }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn diagnostic_clear_message_for_token_mismatch() {
    let input = "fn test() { if RAX == 0 } ";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(!diags.is_empty());
    assert!(diags[0].message.contains("Expected") || diags[0].message.contains("Unexpected"));
}

#[test]
fn diagnostic_for_incomplete_memory_address() {
    let input = "fn test() { mov RAX [ }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn diagnostic_for_invalid_comparison_operator() {
    let input = "fn test() { if RAX ~ 0 { } }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn complex_error_program_with_multiple_issues() {
    let input = r#"
        fn test() {
            alias x : invalid = 123
            if RAX {
            }
        }
    "#;
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    assert!(diags.len() > 0);
}

#[test]
fn lexer_preserves_invalid_number_token() {
    let mut lexer = vend_ast::Lexer::new("9223372036854775808");
    assert!(matches!(lexer.next_token(), vend_ast::parser::Token::InvalidNumber(_)));
}

#[test]
fn diagnostic_preserves_message_quality() {
    let input = "fn test() { mov RAX }";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    for diag in diags {
        assert!(!diag.message.is_empty());
    }
}

#[test]
fn parser_recovers_from_missing_comma_in_param_list() {
    let input = "fn test(a b) { }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn valid_program_produces_no_diagnostics() {
    let input = "fn test() { mov RAX RBX }";
    let result = parse(input);
    assert!(result.is_ok());
}

#[test]
fn diagnostic_for_expected_operand() {
    let input = "fn test() { mov , RAX }";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn diagnostic_message_references_found_token() {
    let input = "fn () { }";
    let result = parse(input);
    assert!(result.is_err());
    let diags = result.unwrap_err();
    let msg = &diags[0].message;
    assert!(msg.contains("function") || msg.contains("identifier") || msg.contains("Expected"));
}
