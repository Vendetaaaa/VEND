use crate::ast::{
    ComparisonOp, Condition, FunctionDecl, Instruction, Operand, Program, ScopeBlock, Statement,
};

pub struct LoweredProgram {
    pub functions: Vec<LoweredFunction>,
}

pub struct LoweredFunction {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<LoweredInstruction>,
}

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
}

impl LoweringContext {
    fn new_label(&mut self, kind: &str) -> String {
        let label = format!("__vend_{}_{}", kind, self.next_label);
        self.next_label += 1;
        label
    }

    fn lower_block(&mut self, block: &ScopeBlock) {
        for statement in &block.statements {
            match statement {
                Statement::AliasDecl(_) => {}
                Statement::Instruction(instruction) => {
                    self.instructions
                        .push(LoweredInstruction::Instruction(instruction.clone()));
                }
                Statement::ScopeBlock(nested) => self.lower_block(nested),
                Statement::If(if_statement) => self.lower_if(
                    &if_statement.condition,
                    &if_statement.then_block,
                    if_statement.else_block.as_ref(),
                ),
                Statement::While(while_statement) => {
                    self.lower_while(&while_statement.condition, &while_statement.body)
                }
            }
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
