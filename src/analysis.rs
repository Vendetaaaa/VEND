use crate::ast::*;
use std::collections::HashMap;
use std::fmt;

/// diagnosis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<Span>,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(
                f,
                "{} (at position {}..{})",
                self.message, span.start, span.end
            ),
            None => write!(f, "{}", self.message),
        }
    }
}

impl Diagnostic {
    /// diag w new comment
    pub fn new(message: impl Into<String>) -> Self {
        Diagnostic {
            message: message.into(),
            span: None,
        }
    }

    /// source span
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn format_with_source(&self, source: &str) -> String {
        match self.span {
            Some(span) => {
                let lines: Vec<&str> = source.lines().collect();
                let mut char_count = 0;
                let mut line_num = 0;
                let mut line_start = 0;
                
                // what line is an error
                for (i, line) in lines.iter().enumerate() {
                    let line_len = line.len() + 1; 
                    if char_count + line_len > span.start {
                        line_num = i;
                        line_start = char_count;
                        break;
                    }
                    char_count += line_len;
                }
                
                let line = lines.get(line_num).unwrap_or(&"");
                let col = span.start - line_start;
                
                format!(
                    "{}:{}:{}  {}\n{}\n{}^",
                    line_num + 1,
                    col + 1,
                    span.end - span.start,
                    self.message,
                    line,
                    " ".repeat(col)
                )
            }
            None => self.message.clone(),
        }
    }
}

/// validates aliasis to prevent overriding
pub fn validate_aliases(block: &ScopeBlock) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut active: Vec<AliasDecl> = Vec::new();

    fn walk(block: &ScopeBlock, active: &mut Vec<AliasDecl>, diags: &mut Vec<Diagnostic>) {
        for stmt in &block.statements {
            match stmt {
                Statement::AliasDecl(a) => {
                    for other in active.iter() {
                        // allowed
                        if a.name == other.name {
                            continue;
                        }
                        if a.physical.overlaps(other.physical) {
                            let msg = format!(
                                "Alias conflict: '{}' ({}) overlaps '{}' ({})",
                                a.name,
                                a.physical,
                                other.name,
                                other.physical
                            );
                            let span = a.span.or(other.span);
                            diags.push(Diagnostic { message: msg, span });
                        }
                    }
                    active.push(a.clone());
                }
                Statement::ScopeBlock(s) => {
                    // new lexical block
                    let before = active.len();
                    walk(s, active, diags);
                    active.truncate(before);
                }
                Statement::If(i) => {
                    // inherance
                    let before = active.len();
                    walk(&i.then_block, active, diags);
                    active.truncate(before);
                    if let Some(else_blk) = &i.else_block {
                        let before2 = active.len();
                        walk(else_blk, active, diags);
                        active.truncate(before2);
                    }
                }
                Statement::While(w) => {
                    let before = active.len();
                    walk(&w.body, active, diags);
                    active.truncate(before);
                }
                Statement::Instruction(_) => {}
            }
        }
    }

    walk(block, &mut active, &mut diags);
    diags
}

// full analysis across program
pub fn analyze_program(prog: &crate::ast::Program) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    for func in &prog.functions {
        let mut func_diags = validate_aliases(&func.body);
        for d in func_diags.drain(..) {
            let msg = format!("In function '{}': {}", func.name, d.message);
            diags.push(Diagnostic { message: msg, span: d.span });
        }
        let mut type_diags = validate_types(&func.body);
        for d in type_diags.drain(..) {
            let msg = format!("In function '{}': {}", func.name, d.message);
            diags.push(Diagnostic { message: msg, span: d.span });
        }
        let mut liveness_diags = analyze_liveness(&func.body).diagnostics;
        for d in liveness_diags.drain(..) {
            let msg = format!("In function '{}': {}", func.name, d.message);
            diags.push(Diagnostic { message: msg, span: d.span });
        }
    }
    diags
}

/// vallidation for the ADI
pub fn validate_types(block: &ScopeBlock) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut active = HashMap::new();
    walk_types(block, &mut active, &mut diagnostics);
    diagnostics
}

/// type check for every run
pub fn analyze_program_types(prog: &crate::ast::Program) -> Vec<Vec<Diagnostic>> {
    prog.functions
        .iter()
        .map(|function| validate_types(&function.body))
        .collect()
}

fn walk_types(
    block: &ScopeBlock,
    active: &mut HashMap<String, Vec<RegisterWidth>>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut declared_names = Vec::new();

    for statement in &block.statements {
        match statement {
            Statement::AliasDecl(alias) => {
                if alias.width != alias.physical.width() {
                    diagnostics.push(Diagnostic {
                        message: format!(
                            "Alias '{}' declares {} but physical register {} is {}",
                            alias.name,
                            alias.width,
                            alias.physical,
                            alias.physical.width()
                        ),
                        span: alias.span,
                    });
                }
                active
                    .entry(alias.name.clone())
                    .or_default()
                    .push(alias.width);
                declared_names.push(alias.name.clone());
            }
            Statement::Instruction(instruction) => {
                for operand in &instruction.operands {
                    validate_operand_types(operand, active, diagnostics);
                }
                validate_instruction_types(instruction, active, diagnostics);
            }
            Statement::If(if_statement) => {
                validate_operand_types(&if_statement.condition.left, active, diagnostics);
                validate_operand_types(&if_statement.condition.right, active, diagnostics);
                validate_condition_types(&if_statement.condition, active, diagnostics);
                walk_types(&if_statement.then_block, active, diagnostics);
                if let Some(else_block) = &if_statement.else_block {
                    walk_types(else_block, active, diagnostics);
                }
            }
            Statement::While(while_statement) => {
                validate_operand_types(&while_statement.condition.left, active, diagnostics);
                validate_operand_types(&while_statement.condition.right, active, diagnostics);
                validate_condition_types(&while_statement.condition, active, diagnostics);
                walk_types(&while_statement.body, active, diagnostics);
            }
            Statement::ScopeBlock(nested) => walk_types(nested, active, diagnostics),
        }
    }

    for name in declared_names.into_iter().rev() {
        let widths = active
            .get_mut(&name)
            .expect("type scope must contain its declared alias");
        widths.pop();
        if widths.is_empty() {
            active.remove(&name);
        }
    }
}

fn validate_operand_types(
    operand: &Operand,
    active: &HashMap<String, Vec<RegisterWidth>>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Operand::MemoryAddress(address) = operand else {
        return;
    };

    for register in address
        .base
        .iter()
        .chain(address.index.iter().map(|index| &index.register))
    {
        if register_width(register, active) != Some(RegisterWidth::Reg64) {
            diagnostics.push(Diagnostic::new(
                "Memory address registers must have reg64 width",
            ));
        }
    }

    if let Some(index) = &address.index {
        if !matches!(index.scale, 1 | 2 | 4 | 8) {
            diagnostics.push(Diagnostic::new(format!(
                "Memory index scale must be 1, 2, 4, or 8, found {}",
                index.scale
            )));
        }
    }
}

fn register_width(
    register: &Register,
    active: &HashMap<String, Vec<RegisterWidth>>,
) -> Option<RegisterWidth> {
    match register {
        Register::Physical(physical) => Some(physical.width()),
        Register::Alias(name) => active
            .get(name)
            .and_then(|widths| widths.last().copied()),
    }
}

fn validate_instruction_types(
    instruction: &Instruction,
    active: &HashMap<String, Vec<RegisterWidth>>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(mnemonic) = supported_width_checked_mnemonic(&instruction.mnemonic.0) else {
        return;
    };

    if instruction.operands.len() < 2 {
        return;
    }

    let (Operand::Register(left), Operand::Register(right)) =
        (&instruction.operands[0], &instruction.operands[1])
    else {
        return;
    };

    let (Some(left_width), Some(right_width)) = (
        register_width(left, active),
        register_width(right, active),
    ) else {
        return;
    };

    if left_width != right_width {
        diagnostics.push(Diagnostic::new(format!(
            "Instruction '{}' requires matching register widths, found {} and {}",
            mnemonic, left_width, right_width
        )));
    }
}

fn supported_width_checked_mnemonic(mnemonic: &str) -> Option<&str> {
    match mnemonic {
        "mov" | "add" | "sub" | "xor" | "cmp" | "test" => Some(mnemonic),
        _ => None,
    }
}

fn validate_condition_types(
    condition: &Condition,
    active: &HashMap<String, Vec<RegisterWidth>>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let (Operand::Register(left), Operand::Register(right)) = (&condition.left, &condition.right)
    else {
        return;
    };

    let (Some(left_width), Some(right_width)) = (
        register_width(left, active),
        register_width(right, active),
    ) else {
        return;
    };

    if left_width != right_width {
        diagnostics.push(Diagnostic::new(format!(
            "Condition requires matching register widths, found {} and {}",
            left_width, right_width
        )));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeInfo {
    pub id: usize,
    pub parent: Option<usize>,
    pub depth: usize,
    pub statement_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasLifetime {
    pub name: Ident,
    pub width: RegisterWidth,
    pub physical: PhysicalRegister,
    pub scope_id: usize,
    pub declaration_index: usize,
    pub end_index: usize,
    pub uses: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivenessReport {
    pub scopes: Vec<ScopeInfo>,
    pub aliases: Vec<AliasLifetime>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn analyze_liveness(block: &ScopeBlock) -> LivenessReport {
    let mut report = LivenessReport {
        scopes: Vec::new(),
        aliases: Vec::new(),
        diagnostics: Vec::new(),
    };
    let mut active = HashMap::new();
    walk_liveness(block, None, 0, &mut active, &mut report);
    report
}

pub fn analyze_program_liveness(prog: &crate::ast::Program) -> Vec<LivenessReport> {
    prog.functions
        .iter()
        .map(|function| analyze_liveness(&function.body))
        .collect()
}

fn walk_liveness(
    block: &ScopeBlock,
    parent: Option<usize>,
    depth: usize,
    active: &mut HashMap<String, Vec<usize>>,
    report: &mut LivenessReport,
) {
    let scope_id = report.scopes.len();
    report.scopes.push(ScopeInfo {
        id: scope_id,
        parent,
        depth,
        statement_count: block.statements.len(),
    });

    let mut declared_names = Vec::new();
    for (statement_index, statement) in block.statements.iter().enumerate() {
        match statement {
            Statement::AliasDecl(alias) => {
                let lifetime = report.aliases.len();
                report.aliases.push(AliasLifetime {
                    name: alias.name.clone(),
                    width: alias.width,
                    physical: alias.physical,
                    scope_id,
                    declaration_index: statement_index,
                    end_index: block.statements.len(),
                    uses: Vec::new(),
                });
                active
                    .entry(alias.name.clone())
                    .or_default()
                    .push(lifetime);
                declared_names.push(alias.name.clone());
            }
            Statement::Instruction(instruction) => {
                for operand in &instruction.operands {
                    record_operand_use(operand, statement_index, active, report);
                }
            }
            Statement::If(if_statement) => {
                record_operand_use(&if_statement.condition.left, statement_index, active, report);
                record_operand_use(&if_statement.condition.right, statement_index, active, report);
                walk_liveness(
                    &if_statement.then_block,
                    Some(scope_id),
                    depth + 1,
                    active,
                    report,
                );
                if let Some(else_block) = &if_statement.else_block {
                    walk_liveness(else_block, Some(scope_id), depth + 1, active, report);
                }
            }
            Statement::While(while_statement) => {
                record_operand_use(&while_statement.condition.left, statement_index, active, report);
                record_operand_use(&while_statement.condition.right, statement_index, active, report);
                walk_liveness(
                    &while_statement.body,
                    Some(scope_id),
                    depth + 1,
                    active,
                    report,
                );
            }
            Statement::ScopeBlock(nested) => {
                walk_liveness(nested, Some(scope_id), depth + 1, active, report);
            }
        }
    }
    for name in declared_names.into_iter().rev() {
        let lifetimes = active
            .get_mut(&name)
            .expect("liveness scope must contain its declared alias");
        lifetimes.pop();
        if lifetimes.is_empty() {
            active.remove(&name);
        }
    }
}

fn record_operand_use(
    operand: &Operand,
    statement_index: usize,
    active: &HashMap<String, Vec<usize>>,
    report: &mut LivenessReport,
) {
    match operand {
        Operand::Register(Register::Alias(name)) => record_alias_use(name, statement_index, active, report),
        Operand::MemoryAddress(address) => {
            if let Some(base) = &address.base {
                record_register_use(base, statement_index, active, report);
            }
            if let Some(index) = &address.index {
                record_register_use(&index.register, statement_index, active, report);
            }
        }
         Operand::Register(Register::Physical(_))
        | Operand::Immediate(_)
        | Operand::Symbol(_) => {}
    }
}

fn record_register_use(
    register: &Register,
    statement_index: usize,
    active: &HashMap<String, Vec<usize>>,
    report: &mut LivenessReport,
) {
    if let Register::Alias(name) = register {
        record_alias_use(name, statement_index, active, report);
    }
}

fn record_alias_use(
    name: &str,
    statement_index: usize,
    active: &HashMap<String, Vec<usize>>,
    report: &mut LivenessReport,
) {
    if let Some(lifetimes) = active.get(name) {
        let lifetime = *lifetimes
            .last()
            .expect("alias stack must contain an active lifetime");
        report.aliases[lifetime].uses.push(statement_index);
    } else {
        report.diagnostics.push(Diagnostic::new(format!(
            "Unknown alias '{}' used in statement {}",
            name, statement_index
        )));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AliasDecl, PhysicalRegister, RegisterWidth, ScopeBlock, Statement};

    #[test]
    fn detects_overlapping_aliases_in_same_block() {
        let a1 = AliasDecl {
            name: "a".into(),
            width: RegisterWidth::Reg64,
            physical: PhysicalRegister::RAX,
            span: None,
        };

        let a2 = AliasDecl {
            name: "b".into(),
            width: RegisterWidth::Reg32,
            physical: PhysicalRegister::EAX,
            span: None,
        };

        let block = ScopeBlock {
            statements: vec![Statement::AliasDecl(a1), Statement::AliasDecl(a2)],
        };

        let diags = validate_aliases(&block);
        assert!(!diags.is_empty(), "expected a conflict diagnostic");
        assert!(diags[0].message.contains("overlaps"));
    }

    #[test]
    fn allows_nonoverlapping_aliases() {
        let a1 = AliasDecl {
            name: "a".into(),
            width: RegisterWidth::Reg64,
            physical: PhysicalRegister::RAX,
            span: None,
        };

        let a2 = AliasDecl {
            name: "b".into(),
            width: RegisterWidth::Reg64,
            physical: PhysicalRegister::RBX,
            span: None,
        };

        let block = ScopeBlock {
            statements: vec![Statement::AliasDecl(a1), Statement::AliasDecl(a2)],
        };

        let diags = validate_aliases(&block);
        assert!(diags.is_empty(), "expected no conflicts");
    }

    #[test]
    fn detects_alias_width_mismatch() {
        let block = ScopeBlock {
            statements: vec![Statement::AliasDecl(AliasDecl {
                name: "value".into(),
                width: RegisterWidth::Reg64,
                physical: PhysicalRegister::EAX,
                span: None,
            })],
        };

        let diags = validate_types(&block);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("declares reg64"));
    }

    #[test]
    fn detects_register_width_mismatch_in_instruction() {
        let block = ScopeBlock {
            statements: vec![Statement::Instruction(Instruction {
                mnemonic: Mnemonic("mov".into()),
                operands: vec![
                    Operand::Register(Register::Physical(PhysicalRegister::RAX)),
                    Operand::Register(Register::Physical(PhysicalRegister::EBX)),
                ],
            })],
        };

        let diags = validate_types(&block);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("matching register widths"));
    }

    #[test]
    fn detects_invalid_memory_register_width_and_scale() {
        let block = ScopeBlock {
            statements: vec![Statement::Instruction(Instruction {
                mnemonic: Mnemonic("mov".into()),
                operands: vec![
                    Operand::Register(Register::Physical(PhysicalRegister::RAX)),
                    Operand::MemoryAddress(MemoryAddress {
                        base: Some(Register::Physical(PhysicalRegister::EAX)),
                        index: Some(IndexedRegister {
                            register: Register::Physical(PhysicalRegister::RBX),
                            scale: 3,
                        }),
                        displacement: 0,
                    }),
                ],
            })],
        };

        let diags = validate_types(&block);
        assert_eq!(diags.len(), 2);
        assert!(diags[0].message.contains("reg64 width"));
        assert!(diags[1].message.contains("scale"));
    }

    #[test]
    fn allows_matching_register_widths_in_conditions() {
        let block = ScopeBlock {
            statements: vec![Statement::If(If {
                condition: Condition {
                    left: Operand::Register(Register::Physical(PhysicalRegister::EAX)),
                    op: ComparisonOp::Eq,
                    right: Operand::Register(Register::Physical(PhysicalRegister::EBX)),
                },
                then_block: ScopeBlock { statements: vec![] },
                else_block: None,
            })],
        };

        assert!(validate_types(&block).is_empty());
    }

    #[test]
    fn detects_conflict_with_parent_scope_alias() {
        let parent = AliasDecl {
            name: "p".into(),
            width: RegisterWidth::Reg64,
            physical: PhysicalRegister::RAX,
            span: None,
        };

        let child = AliasDecl {
            name: "c".into(),
            width: RegisterWidth::Reg16,
            physical: PhysicalRegister::AX,
            span: None,
        };

        let child_block = ScopeBlock {
            statements: vec![Statement::AliasDecl(child)],
        };

        let block = ScopeBlock {
            statements: vec![Statement::AliasDecl(parent), Statement::ScopeBlock(child_block)],
        };

        let diags = validate_aliases(&block);
        assert!(!diags.is_empty(), "expected conflict between parent and child alias");
    }

    
    #[test]
    fn tracks_alias_lifetimes_and_nested_scope_visibility() {
        let block = ScopeBlock {
            statements: vec![
                Statement::AliasDecl(AliasDecl {
                    name: "parent".into(),
                    width: RegisterWidth::Reg64,
                    physical: PhysicalRegister::RAX,
                    span: None,
                }),
                Statement::ScopeBlock(ScopeBlock {
                    statements: vec![
                        Statement::Instruction(Instruction {
                            mnemonic: Mnemonic("mov".into()),
                            operands: vec![Operand::Register(Register::Alias("parent".into()))],
                        }),
                        Statement::AliasDecl(AliasDecl {
                            name: "child".into(),
                            width: RegisterWidth::Reg32,
                            physical: PhysicalRegister::RBX,
                            span: None,
                        }),
                    ],
                }),
                Statement::Instruction(Instruction {
                    mnemonic: Mnemonic("mov".into()),
                    operands: vec![Operand::Register(Register::Alias("child".into()))],
                }),
            ],
        };

        let report = analyze_liveness(&block);
        assert_eq!(report.scopes.len(), 2);
        assert_eq!(report.scopes[1].parent, Some(0));
        assert_eq!(report.aliases.len(), 2);
        assert_eq!(report.aliases[0].declaration_index, 0);
        assert_eq!(report.aliases[0].end_index, 3);
        assert_eq!(report.aliases[0].uses, vec![0]);
        assert_eq!(report.aliases[1].scope_id, 1);
        assert!(report.aliases[1].uses.is_empty());
        assert_eq!(report.diagnostics.len(), 1);
        assert!(report.diagnostics[0].message.contains("Unknown alias 'child'"));
    }

    #[test]
    fn program_analysis_reports_unknown_aliases() {
        let program = Program {
            functions: vec![FunctionDecl {
                name: "main".into(),
                params: Vec::new(),
                body: ScopeBlock {
                    statements: vec![Statement::Instruction(Instruction {
                        mnemonic: Mnemonic("mov".into()),
                        operands: vec![Operand::Register(Register::Alias("missing".into()))],
                    })],
                },
            }],
        };

        let diagnostics = analyze_program(&program);
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("In function 'main'"));
        assert!(diagnostics[0].message.contains("Unknown alias 'missing'"));
    }
}
