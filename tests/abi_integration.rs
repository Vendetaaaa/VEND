use vend_ast::{
    emit_function, lower_function, parse, validate_abi, Abi, CallingConvention, PhysicalRegister,
};

#[test]
fn emits_external_call_symbols_without_treating_them_as_aliases() {
    let program = parse("fn main() { call puts }").expect("source should parse");
    let assembly = emit_function(&lower_function(&program.functions[0])).expect("call should emit");

    assert!(assembly.contains("call puts"));
}

#[test]
fn exposes_platform_calling_conventions() {
    let system_v = Abi::system_v();
    assert_eq!(system_v.convention, CallingConvention::SystemV);
    assert_eq!(system_v.argument_registers[0], PhysicalRegister::RDI);
    assert_eq!(system_v.shadow_space, 0);

    let windows = Abi::windows_x64();
    assert_eq!(windows.convention, CallingConvention::WindowsX64);
    assert_eq!(windows.argument_registers[0], PhysicalRegister::RCX);
    assert_eq!(windows.shadow_space, 32);
}

#[test]
fn rejects_call_shapes_that_cannot_describe_an_external_abi_boundary() {
    let program = parse("fn main() { call RAX, RBX }").expect("source should parse");
    let diagnostics = validate_abi(&program.functions[0], Abi::system_v());

    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("external symbol"));
}
