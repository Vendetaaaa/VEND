use crate::ast::*;
use std::fmt;

/// diagnosis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<Span>,
}

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
    let mut active: Vec<&AliasDecl> = Vec::new();

    fn walk(block: &ScopeBlock, active: &mut Vec<&AliasDecl>, diags: &mut Vec<Diagnostic>) {
        for stmt in &block.statements {
            match stmt {
                Statement::AliasDecl(a) => {
                    for &other in active.iter() {
                        // Allow shadows for aliases
                        if a name == other.name {
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
                    active.push(a);
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
        let mut liveness_diags = analyze_liveness(&func.body).diagnostics;
        for d in liveness_diags.drain(..) {
            let msg = format!("In function '{}': {}", func.name, d.message);
            diags.push(Diagnostic { message: msg, span: d.span });
        }
    }
    diags
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

struct ActiveAlias {
    name: Ident,
    lifetime: usize,
}

pub fn analyze_liveness(block: &ScopeBlock) -> LivenessReport {
    let mut report = LivenessReport {
        scopes: Vec::new(),
        aliases: Vec::new(),
        diagnostics: Vec::new(),
    };
    let mut active = Vec::new();
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
    active: &mut Vec<ActiveAlias>,
    report: &mut LivenessReport,
) {
    let scope_id = report.scopes.len();
    report.scopes.push(ScopeInfo {
        id: scope_id,
        parent,
        depth,
        statement_count: block.statements.len(),
    });

    let active_before = active.len();
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
                active.push(ActiveAlias {
                    name: alias.name.clone(),
                    lifetime,
                });
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
    active.truncate(active_before);
}

fn record_operand_use(
    operand: &Operand,
    statement_index: usize,
    active: &[ActiveAlias],
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
        Operand::Register(Register::Physical(_)) | Operand::Immediate(_) => {}
    }
}

fn record_register_use(
    register: &Register,
    statement_index: usize,
    active: &[ActiveAlias],
    report: &mut LivenessReport,
) {
    if let Register::Alias(name) = register {
        record_alias_use(name, statement_index, active, report);
    }
}

fn record_alias_use(
    name: &str,
    statement_index: usize,
    active: &[ActiveAlias],
    report: &mut LivenessReport,
) {
    if let Some(alias) = active.iter().rev().find(|alias| alias.name == name) {
        report.aliases[alias.lifetime].uses.push(statement_index);
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
}
