pub const VEND_GRAMMAR: &str = r#"
Program       -> FunctionDecl*
FunctionDecl  -> 'fn' Ident '(' ParamList? ')' '{' ScopeBlock '}'
ParamList     -> Ident (',' Ident)*
ScopeBlock    -> '{' Statement* '}'
Statement     -> AliasDecl | Instruction | If | While | ScopeBlock
AliasDecl     -> 'alias' Ident ':' RegType '=' PhysReg
Instruction   -> Mnemonic Operand (',' Operand)?
If            -> 'if' Condition ScopeBlock ('else' ScopeBlock)?
While         -> 'while' Condition ScopeBlock
Condition     -> Operand CmpOp Operand
Operand       -> Register | Immediate | MemoryAddress
Register      -> PhysReg | Ident
Immediate     -> SignedInteger
MemoryAddress -> '[' Base? ('+' Index)? ('+' Disp)? ']'
Base          -> Register
Index         -> Register '*' UnsignedInteger
Disp          -> SignedInteger
RegType       -> 'reg64' | 'reg32' | 'reg16' | 'reg8'
PhysReg       -> RAX | EAX | AX | AL | AH
              | RBX | EBX | BX | BL | BH
              | RCX | ECX | CX | CL | CH
              | RDX | EDX | DX | DL | DH
              | RSI | ESI | SI | SIL
              | RDI | EDI | DI | DIL
              | RBP | EBP | BP | BPL
              | RSP | ESP | SP | SPL
              | R8 | R8D | R8W | R8B
              | R9 | R9D | R9W | R9B
              | R10 | R10D | R10W | R10B
              | R11 | R11D | R11W | R11B
              | R12 | R12D | R12W | R12B
              | R13 | R13D | R13W | R13B
              | R14 | R14D | R14W | R14B
              | R15 | R15D | R15W | R15B
"#;
