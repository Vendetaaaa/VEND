use crate::analysis::Diagnostic;
use crate::ast::{FunctionDecl, PhysicalRegister, Statement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    SystemV,
    WindowsX64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Abi {
    pub convention: CallingConvention,
    pub argument_registers: &'static [PhysicalRegister],
    pub return_register: PhysicalRegister,
    pub caller_saved_registers: &'static [PhysicalRegister],
    pub stack_alignment: u8,
    pub shadow_space: u8,
}

const SYSTEM_V_ARGUMENTS: &[PhysicalRegister] = &[
    PhysicalRegister::RDI,
    PhysicalRegister::RSI,
    PhysicalRegister::RDX,
    PhysicalRegister::RCX,
    PhysicalRegister::R8,
    PhysicalRegister::R9,
];

const WINDOWS_ARGUMENTS: &[PhysicalRegister] = &[
    PhysicalRegister::RCX,
    PhysicalRegister::RDX,
    PhysicalRegister::R8,
    PhysicalRegister::R9,
];

const SYSTEM_V_CALLER_SAVED: &[PhysicalRegister] = &[
    PhysicalRegister::RAX,
    PhysicalRegister::RCX,
    PhysicalRegister::RDX,
    PhysicalRegister::RSI,
    PhysicalRegister::RDI,
    PhysicalRegister::R8,
    PhysicalRegister::R9,
    PhysicalRegister::R10,
    PhysicalRegister::R11,
];

const WINDOWS_CALLER_SAVED: &[PhysicalRegister] = &[
    PhysicalRegister::RAX,
    PhysicalRegister::RCX,
    PhysicalRegister::RDX,
    PhysicalRegister::R8,
    PhysicalRegister::R9,
    PhysicalRegister::R10,
    PhysicalRegister::R11,
];

impl Abi {
    pub const fn system_v() -> Self {
        Self {
            convention: CallingConvention::SystemV,
            argument_registers: SYSTEM_V_ARGUMENTS,
            return_register: PhysicalRegister::RAX,
            caller_saved_registers: SYSTEM_V_CALLER_SAVED,
            stack_alignment: 16,
            shadow_space: 0,
        }
    }

    pub const fn windows_x64() -> Self {
        Self {
            convention: CallingConvention::WindowsX64,
            argument_registers: WINDOWS_ARGUMENTS,
            return_register: PhysicalRegister::RAX,
            caller_saved_registers: WINDOWS_CALLER_SAVED,
            stack_alignment: 16,
            shadow_space: 32,
        }
    }
}

pub fn validate_abi(function: &FunctionDecl, abi: Abi) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_block(&function.body.statements, abi, &mut diagnostics);
    diagnostics
}

fn validate_block(statements: &[Statement], _abi: Abi, diagnostics: &mut Vec<Diagnostic>) {
    for statement in statements {
        match statement {
            Statement::Instruction(instruction) if instruction.mnemonic.0 == "call" => {
                if instruction.operands.len() != 1
                    || !matches!(
                        instruction.operands.first(),
                        Some(crate::ast::Operand::Symbol(_))
                    )
                {
                    diagnostics.push(Diagnostic::new(
                        "ABI call expects exactly one external symbol, for example 'call puts'",
                    ));
                }
            }
            Statement::If(if_statement) => {
                validate_block(&if_statement.then_block.statements, _abi, diagnostics);
                if let Some(else_block) = &if_statement.else_block {
                    validate_block(&else_block.statements, _abi, diagnostics);
                }
            }
            Statement::While(while_statement) => {
                validate_block(&while_statement.body.statements, _abi, diagnostics);
            }
            Statement::ScopeBlock(block) => validate_block(&block.statements, _abi, diagnostics),
            Statement::AliasDecl(_) | Statement::Instruction(_) => {}
        }
    }
}
