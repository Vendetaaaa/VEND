use crate::ast::{
    ComparisonOp, Condition, FunctionDecl, Instruction, Operand, PhysicalRegister, Program,
    Register, ScopeBlock, Statement,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredProgram {
    pub functions: Vec<LoweredFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredFunction {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<LoweredInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredInstruction {
    Label(String),
    Instruction(Instruction),
    Compare {
        left: Operand,
        right: Operand,
    },
    Branch {
        condition: ComparisonOp,
        target: String,
    },
    Jump {
        target: String,
    },
}

pub fn lower_program(program: &Program) -> LoweredProgram {
    LoweredProgram {
        functions: program.functions.iter().map(lower_function).collect(),
    }
}

pub fn lower_function(function: &FunctionDecl) -> LoweredFunction {
    let mut context = LoweringContext {
        next_label: 0,
        instructions: Vec::new(),
        aliases: HashMap::new(),
    };
    context.lower_block(&function.body);

    LoweredFunction {
        name: function.name.clone(),
        params: function.params.clone(),
        instructions: context.instructions,
    }
}

struct LoweringContext {
    next_label: usize,
    instructions: Vec<LoweredInstruction>,
    aliases: HashMap<String, Vec<PhysicalRegister>>,
}

impl LoweringContext {
    fn new_label(&mut self, kind: &str) -> String {
        let label = format!("__vend_{}_{}", kind, self.next_label);
        self.next_label += 1;
        label
    }

    fn lower_block(&mut self, block: &ScopeBlock) {
        let mut declared_aliases = Vec::new();
        for statement in &block.statements {
            match statement {
                Statement::AliasDecl(alias) => {
                    self.aliases
                        .entry(alias.name.clone())
                        .or_default()
                        .push(alias.physical);
                    declared_aliases.push(alias.name.clone());
                }
                Statement::Instruction(instruction) => {
                    self.instructions.push(LoweredInstruction::Instruction(
                        self.resolve_instruction(instruction),
                    ));
                }
                Statement::ScopeBlock(nested) => self.lower_block(nested),
                Statement::If(if_statement) => self.lower_if(
                    &self.resolve_condition(&if_statement.condition),
                    &if_statement.then_block,
                    if_statement.else_block.as_ref(),
                ),
                Statement::While(while_statement) => {
                    let condition = self.resolve_condition(&while_statement.condition);
                    self.lower_while(&condition, &while_statement.body)
                }
            }
        }
        for name in declared_aliases.into_iter().rev() {
            let aliases = self
                .aliases
                .get_mut(&name)
                .expect("lowering scope must contain its declared alias");
            aliases.pop();
            if aliases.is_empty() {
                self.aliases.remove(&name);
            }
        }
    }

    fn resolve_instruction(&self, instruction: &Instruction) -> Instruction {
        Instruction {
            mnemonic: instruction.mnemonic.clone(),
            operands: instruction
                .operands
                .iter()
                .map(|operand| self.resolve_operand(operand))
                .collect(),
        }
    }

    fn resolve_condition(&self, condition: &Condition) -> Condition {
        Condition {
            left: self.resolve_operand(&condition.left),
            op: condition.op,
            right: self.resolve_operand(&condition.right),
        }
    }

    fn resolve_operand(&self, operand: &Operand) -> Operand {
        match operand {
            Operand::Register(Register::Alias(name)) => self
                .aliases
                .get(name)
                .and_then(|registers| registers.last().copied())
                .map(Register::Physical)
                .map(Operand::Register)
                .unwrap_or_else(|| operand.clone()),
            Operand::MemoryAddress(address) => {
                let mut address = address.clone();
                address.base = address
                    .base
                    .as_ref()
                    .map(|register| self.resolve_register(register));
                address.index = address
                    .index
                    .as_ref()
                    .map(|index| crate::ast::IndexedRegister {
                        register: self.resolve_register(&index.register),
                        scale: index.scale,
                    });
                Operand::MemoryAddress(address)
            }
            _ => operand.clone(),
        }
    }

    fn resolve_register(&self, register: &Register) -> Register {
        match self.resolve_operand(&Operand::Register(register.clone())) {
            Operand::Register(register) => register,
            _ => unreachable!("register resolution must return a register"),
        }
    }

    fn lower_if(
        &mut self,
        condition: &Condition,
        then_block: &ScopeBlock,
        else_block: Option<&ScopeBlock>,
    ) {
        let then_label = self.new_label("if_then");
        let end_label = self.new_label("if_end");
        let else_label = else_block.map(|_| self.new_label("if_else"));

        self.emit_condition(condition);
        self.instructions.push(LoweredInstruction::Branch {
            condition: condition.op,
            target: then_label.clone(),
        });
        self.instructions.push(LoweredInstruction::Jump {
            target: else_label.clone().unwrap_or_else(|| end_label.clone()),
        });
        self.instructions
            .push(LoweredInstruction::Label(then_label));
        self.lower_block(then_block);

        if let Some(else_block) = else_block {
            self.instructions.push(LoweredInstruction::Jump {
                target: end_label.clone(),
            });
            self.instructions.push(LoweredInstruction::Label(
                else_label.expect("else label exists"),
            ));
            self.lower_block(else_block);
        }

        self.instructions.push(LoweredInstruction::Label(end_label));
    }

    fn lower_while(&mut self, condition: &Condition, body: &ScopeBlock) {
        let head_label = self.new_label("while_head");
        let body_label = self.new_label("while_body");
        let end_label = self.new_label("while_end");

        self.instructions
            .push(LoweredInstruction::Label(head_label.clone()));
        self.emit_condition(condition);
        self.instructions.push(LoweredInstruction::Branch {
            condition: condition.op,
            target: body_label.clone(),
        });
        self.instructions.push(LoweredInstruction::Jump {
            target: end_label.clone(),
        });
        self.instructions
            .push(LoweredInstruction::Label(body_label));
        self.lower_block(body);
        self.instructions
            .push(LoweredInstruction::Jump { target: head_label });
        self.instructions.push(LoweredInstruction::Label(end_label));
    }

    fn emit_condition(&mut self, condition: &Condition) {
        self.instructions.push(LoweredInstruction::Compare {
            left: condition.left.clone(),
            right: condition.right.clone(),
        });
    }
}
