use crate::ast::{ComparisonOp, MemoryAddress, Operand, Register};
use crate::lowering::{LoweredFunction, LoweredInstruction, LoweredProgram};
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmissionError {
    pub message: String,
}

impl std::fmt::Display for EmissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for EmissionError {}

pub fn emit_program(program: &LoweredProgram) -> Result<String, EmissionError> {
    let mut output = String::from(".intel_syntax noprefix\n.text\n");
    for (index, function) in program.functions.iter().enumerate() {
        if index != 0 {
            output.push('\n');
        }
        emit_function_into(function, &mut output)?;
    }
    Ok(output)
}

pub fn emit_function(function: &LoweredFunction) -> Result<String, EmissionError> {
    let mut output = String::from(".intel_syntax noprefix\n.text\n");
    emit_function_into(function, &mut output)?;
    Ok(output)
}

fn emit_function_into(
    function: &LoweredFunction,
    output: &mut String,
) -> Result<(), EmissionError> {
    writeln!(output, ".globl {}", function.name).unwrap();
    writeln!(output, "{}:", function.name).unwrap();
    for instruction in &function.instructions {
        match instruction {
            LoweredInstruction::Label(label) => {
                writeln!(output, "{}:", label).unwrap();
            }
            LoweredInstruction::Instruction(instruction) => {
                write!(output, "    {}", instruction.mnemonic.0).unwrap();
                emit_operands(&instruction.operands, output)?;
                output.push('\n');
            }
            LoweredInstruction::Compare { left, right } => {
                write!(output, "    cmp ").unwrap();
                write_operand(left, output)?;
                output.push_str(", ");
                write_operand(right, output)?;
                output.push('\n');
            }
            LoweredInstruction::Branch { condition, target } => {
                writeln!(output, "    {} {}", jump_mnemonic(*condition), target).unwrap();
            }
            LoweredInstruction::Jump { target } => {
                writeln!(output, "    jmp {}", target).unwrap();
            }
        }
    }
    Ok(())
}

fn emit_operands(operands: &[Operand], output: &mut String) -> Result<(), EmissionError> {
    if let Some((first, rest)) = operands.split_first() {
        output.push(' ');
        write_operand(first, output)?;
        for operand in rest {
            output.push_str(", ");
            write_operand(operand, output)?;
        }
    }
    Ok(())
}

fn write_operand(operand: &Operand, output: &mut String) -> Result<(), EmissionError> {
    match operand {
        Operand::Register(Register::Physical(register)) => {
            output.push_str(register.as_str());
            Ok(())
        }
        Operand::Register(Register::Alias(name)) => Err(EmissionError {
            message: format!("cannot emit unresolved alias '{}'", name),
        }),
        Operand::Immediate(value) => {
            write!(output, "{}", value).unwrap();
            Ok(())
        }
        Operand::MemoryAddress(address) => write_memory_address(address, output),
    }
}

fn write_memory_address(address: &MemoryAddress, output: &mut String) -> Result<(), EmissionError> {
    output.push('[');
    let mut has_term = false;
    if let Some(base) = &address.base {
        write_register(base, output)?;
        has_term = true;
    }
    if let Some(index) = &address.index {
        if has_term {
            output.push_str(" + ");
        }
        write_register(&index.register, output)?;
        write!(output, "*{}", index.scale).unwrap();
        has_term = true;
    }
    if address.displacement != 0 || !has_term {
        if has_term {
            if address.displacement < 0 {
                write!(output, " - {}", address.displacement.unsigned_abs()).unwrap();
            } else {
                write!(output, " + {}", address.displacement).unwrap();
            }
        } else {
            write!(output, "{}", address.displacement).unwrap();
        }
    }
    output.push(']');
    Ok(())
}

fn write_register(register: &Register, output: &mut String) -> Result<(), EmissionError> {
    match register {
        Register::Physical(register) => {
            output.push_str(register.as_str());
            Ok(())
        }
        Register::Alias(name) => Err(EmissionError {
            message: format!("cannot emit unresolved alias '{}'", name),
        }),
    }
}

fn jump_mnemonic(condition: ComparisonOp) -> &'static str {
    match condition {
        ComparisonOp::Eq => "je",
        ComparisonOp::Ne => "jne",
        ComparisonOp::Lt => "jl",
        ComparisonOp::Le => "jle",
        ComparisonOp::Gt => "jg",
        ComparisonOp::Ge => "jge",
    }
}
