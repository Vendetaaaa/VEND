use crate::ast::*;

/// diagnosis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic(pub String);

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
                            diags.push(Diagnostic(msg));
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
            let msg = format!("In function '{}': {}", func.name, d.0);
            diags.push(Diagnostic(msg));
        }
    }
    diags
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
        };

        let a2 = AliasDecl {
            name: "b".into(),
            width: RegisterWidth::Reg32,
            physical: PhysicalRegister::EAX,
        };

        let block = ScopeBlock {
            statements: vec![Statement::AliasDecl(a1), Statement::AliasDecl(a2)],
        };

        let diags = validate_aliases(&block);
        assert!(!diags.is_empty(), "expected a conflict diagnostic");
        assert!(diags[0].0.contains("overlaps"));
    }

    #[test]
    fn allows_nonoverlapping_aliases() {
        let a1 = AliasDecl {
            name: "a".into(),
            width: RegisterWidth::Reg64,
            physical: PhysicalRegister::RAX,
        };

        let a2 = AliasDecl {
            name: "b".into(),
            width: RegisterWidth::Reg64,
            physical: PhysicalRegister::RBX,
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
        };

        let child = AliasDecl {
            name: "c".into(),
            width: RegisterWidth::Reg16,
            physical: PhysicalRegister::AX,
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
}
