use std::fmt;

pub type Ident = String;

// atm just a list off functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
}

// functions are one level and one pm
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDecl {
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: ScopeBlock,
}

// scope block are keys for alias
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeBlock {
    pub statements: Vec<Statement>,
}

// sts are explicit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    AliasDecl(AliasDecl),
    Instruction(Instruction),
    If(If),
    While(While),
    ScopeBlock(ScopeBlock),
}

// alias read-friendly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasDecl {
    pub name: Ident,
    pub width: RegisterWidth,
    pub physical: PhysicalRegister,
}

// for now it stays legal
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}

// mnems 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mnemonic(pub Ident);

// the big 3 dawgs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    MemoryAddress(MemoryAddress),
}

// phis or lexical
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Register {
    Physical(PhysicalRegister),
    Alias(Ident),
}

// hardware verify
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterWidth {
    Reg64,
    Reg32,
    Reg16,
    Reg8,
}

impl RegisterWidth {
    // compare the bits by width
    pub const fn bits(self) -> u8 {
        match self {
            Self::Reg64 => 64,
            Self::Reg32 => 32,
            Self::Reg16 => 16,
            Self::Reg8 => 8,
        }
    }

    // read-friendly form
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reg64 => "reg64",
            Self::Reg32 => "reg32",
            Self::Reg16 => "reg16",
            Self::Reg8 => "reg8",
        }
    }
}

impl fmt::Display for RegisterWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// to be modified, atm is explicit
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalRegister {
    RAX,
    EAX,
    AX,
    AL,
    AH,
    RBX,
    EBX,
    BX,
    BL,
    BH,
    RCX,
    ECX,
    CX,
    CL,
    CH,
    RDX,
    EDX,
    DX,
    DL,
    DH,
    RSI,
    ESI,
    SI,
    SIL,
    RDI,
    EDI,
    DI,
    DIL,
    RBP,
    EBP,
    BP,
    BPL,
    RSP,
    ESP,
    SP,
    SPL,
    R8,
    R8D,
    R8W,
    R8B,
    R9,
    R9D,
    R9W,
    R9B,
    R10,
    R10D,
    R10W,
    R10B,
    R11,
    R11D,
    R11W,
    R11B,
    R12,
    R12D,
    R12W,
    R12B,
    R13,
    R13D,
    R13W,
    R13B,
    R14,
    R14D,
    R14W,
    R14B,
    R15,
    R15D,
    R15W,
    R15B,
}

impl PhysicalRegister {
    // width detection
    pub const fn width(self) -> RegisterWidth {
        match self {
            Self::RAX | Self::RBX | Self::RCX | Self::RDX | Self::RSI | Self::RDI | Self::RBP | Self::RSP
            | Self::R8 | Self::R9 | Self::R10 | Self::R11 | Self::R12 | Self::R13 | Self::R14 | Self::R15 => {
                RegisterWidth::Reg64
            }
            Self::EAX | Self::EBX | Self::ECX | Self::EDX | Self::ESI | Self::EDI | Self::EBP | Self::ESP
            | Self::R8D | Self::R9D | Self::R10D | Self::R11D | Self::R12D | Self::R13D | Self::R14D | Self::R15D => {
                RegisterWidth::Reg32
            }
            Self::AX | Self::BX | Self::CX | Self::DX | Self::SI | Self::DI | Self::BP | Self::SP
            | Self::R8W | Self::R9W | Self::R10W | Self::R11W | Self::R12W | Self::R13W | Self::R14W | Self::R15W => {
                RegisterWidth::Reg16
            }
            _ => RegisterWidth::Reg8,
        }
    }

    // output is predictable
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RAX => "RAX",
            Self::EAX => "EAX",
            Self::AX => "AX",
            Self::AL => "AL",
            Self::AH => "AH",
            Self::RBX => "RBX",
            Self::EBX => "EBX",
            Self::BX => "BX",
            Self::BL => "BL",
            Self::BH => "BH",
            Self::RCX => "RCX",
            Self::ECX => "ECX",
            Self::CX => "CX",
            Self::CL => "CL",
            Self::CH => "CH",
            Self::RDX => "RDX",
            Self::EDX => "EDX",
            Self::DX => "DX",
            Self::DL => "DL",
            Self::DH => "DH",
            Self::RSI => "RSI",
            Self::ESI => "ESI",
            Self::SI => "SI",
            Self::SIL => "SIL",
            Self::RDI => "RDI",
            Self::EDI => "EDI",
            Self::DI => "DI",
            Self::DIL => "DIL",
            Self::RBP => "RBP",
            Self::EBP => "EBP",
            Self::BP => "BP",
            Self::BPL => "BPL",
            Self::RSP => "RSP",
            Self::ESP => "ESP",
            Self::SP => "SP",
            Self::SPL => "SPL",
            Self::R8 => "R8",
            Self::R8D => "R8D",
            Self::R8W => "R8W",
            Self::R8B => "R8B",
            Self::R9 => "R9",
            Self::R9D => "R9D",
            Self::R9W => "R9W",
            Self::R9B => "R9B",
            Self::R10 => "R10",
            Self::R10D => "R10D",
            Self::R10W => "R10W",
            Self::R10B => "R10B",
            Self::R11 => "R11",
            Self::R11D => "R11D",
            Self::R11W => "R11W",
            Self::R11B => "R11B",
            Self::R12 => "R12",
            Self::R12D => "R12D",
            Self::R12W => "R12W",
            Self::R12B => "R12B",
            Self::R13 => "R13",
            Self::R13D => "R13D",
            Self::R13W => "R13W",
            Self::R13B => "R13B",
            Self::R14 => "R14",
            Self::R14D => "R14D",
            Self::R14W => "R14W",
            Self::R14B => "R14B",
            Self::R15 => "R15",
            Self::R15D => "R15D",
            Self::R15W => "R15W",
            Self::R15B => "R15B",
        }
    }
}

impl fmt::Display for PhysicalRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// structures
// base && index && displ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryAddress {
    pub base: Option<Register>,
    pub index: Option<IndexedRegister>,
    pub displacement: i64,
}

// index carries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedRegister {
    pub register: Register,
    pub scale: u8,
}

// if statment
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub condition: Condition,
    pub then_block: ScopeBlock,
    pub else_block: Option<ScopeBlock>,
}

// while loop
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While {
    pub condition: Condition,
    pub body: ScopeBlock,
}

// conditin for if / while
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub left: Operand,
    pub op: ComparisonOp,
    pub right: Operand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
