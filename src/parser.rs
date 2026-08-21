use crate::ast::*;
use std::fmt;

/// token by lexer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // keywords
    Fn,
    Alias,
    If,
    Else,
    While,
    
    // ids
    Ident(String),
    Integer(i64),
    
    PhysReg(PhysicalRegister),
    
    // types ^^
    Reg64,
    Reg32,
    Reg16,
    Reg8,
    
    // oprs and puncts
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Equal,
    Plus,
    Minus,
    Star,
    
    // comparsaions
    EqEq,      // ==
    NotEq,     // !=
    Lt,        // <
    Gt,        // >
    LtEq,      // <=
    GtEq,      // >=
    
    // end of input
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Fn => write!(f, "'fn'"),
            Token::Alias => write!(f, "'alias'"),
            Token::If => write!(f, "'if'"),
            Token::Else => write!(f, "'else'"),
            Token::While => write!(f, "'while'"),
            Token::Ident(s) => write!(f, "identifier '{}'", s),
            Token::Integer(n) => write!(f, "integer {}", n),
            Token::PhysReg(r) => write!(f, "physical register '{}'", r),
            Token::Reg64 => write!(f, "'reg64'"),
            Token::Reg32 => write!(f, "'reg32'"),
            Token::Reg16 => write!(f, "'reg16'"),
            Token::Reg8 => write!(f, "'reg8'"),
            Token::LParen => write!(f, "'('"),
            Token::RParen => write!(f, "')'"),
            Token::LBrace => write!(f, "'{{'"),
            Token::RBrace => write!(f, "'}}'"),
            Token::LBracket => write!(f, "'['"),
            Token::RBracket => write!(f, "']'"),
            Token::Comma => write!(f, "','"),
            Token::Colon => write!(f, "':'"),
            Token::Equal => write!(f, "'='"),
            Token::Plus => write!(f, "'+'"),
            Token::Minus => write!(f, "'-'"),
            Token::Star => write!(f, "'*'"),
            Token::EqEq => write!(f, "'=='"),
            Token::NotEq => write!(f, "'!='"),
            Token::Lt => write!(f, "'<'"),
            Token::Gt => write!(f, "'>'"),
            Token::LtEq => write!(f, "'<='"),
            Token::GtEq => write!(f, "'>='"),
            Token::Eof => write!(f, "end of input"),
        }
    }
}

/// lexer
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current_pos(&self) -> usize {
        self.pos
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn peek_ahead(&self, offset: usize) -> Option<char> {
        let pos = self.pos + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.input.len() {
            let ch = self.input[self.pos];
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // comment is "//"
        if self.peek() == Some('/') && self.peek_ahead(1) == Some('/') {
            self.advance();
            self.advance();
            while let Some(ch) = self.advance() {
                if ch == '\n' {
                    break;
                }
            }
        }
    }

    fn read_ident(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn read_number(&mut self) -> i64 {
        let mut result = String::new();
        let negative = if self.peek() == Some('-') {
            self.advance();
            true
        } else {
            false
        };

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let num = result.parse::<i64>().unwrap_or(0);
        if negative {
            -num
        } else {
            num
        }
    }

    fn ident_to_token(&self, ident: &str) -> Token {
        match ident {
            "fn" => Token::Fn,
            "alias" => Token::Alias,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "reg64" => Token::Reg64,
            "reg32" => Token::Reg32,
            "reg16" => Token::Reg16,
            "reg8" => Token::Reg8,
            _ => {
                // parse as psy reg
                if let Ok(reg) = self.parse_physical_register(ident) {
                    Token::PhysReg(reg)
                } else {
                    Token::Ident(ident.to_string())
                }
            }
        }
    }

    fn parse_physical_register(&self, s: &str) -> Result<PhysicalRegister, ()> {
        match s {
            "RAX" => Ok(PhysicalRegister::RAX),
            "EAX" => Ok(PhysicalRegister::EAX),
            "AX" => Ok(PhysicalRegister::AX),
            "AL" => Ok(PhysicalRegister::AL),
            "AH" => Ok(PhysicalRegister::AH),
            "RBX" => Ok(PhysicalRegister::RBX),
            "EBX" => Ok(PhysicalRegister::EBX),
            "BX" => Ok(PhysicalRegister::BX),
            "BL" => Ok(PhysicalRegister::BL),
            "BH" => Ok(PhysicalRegister::BH),
            "RCX" => Ok(PhysicalRegister::RCX),
            "ECX" => Ok(PhysicalRegister::ECX),
            "CX" => Ok(PhysicalRegister::CX),
            "CL" => Ok(PhysicalRegister::CL),
            "CH" => Ok(PhysicalRegister::CH),
            "RDX" => Ok(PhysicalRegister::RDX),
            "EDX" => Ok(PhysicalRegister::EDX),
            "DX" => Ok(PhysicalRegister::DX),
            "DL" => Ok(PhysicalRegister::DL),
            "DH" => Ok(PhysicalRegister::DH),
            "RSI" => Ok(PhysicalRegister::RSI),
            "ESI" => Ok(PhysicalRegister::ESI),
            "SI" => Ok(PhysicalRegister::SI),
            "SIL" => Ok(PhysicalRegister::SIL),
            "RDI" => Ok(PhysicalRegister::RDI),
            "EDI" => Ok(PhysicalRegister::EDI),
            "DI" => Ok(PhysicalRegister::DI),
            "DIL" => Ok(PhysicalRegister::DIL),
            "RBP" => Ok(PhysicalRegister::RBP),
            "EBP" => Ok(PhysicalRegister::EBP),
            "BP" => Ok(PhysicalRegister::BP),
            "BPL" => Ok(PhysicalRegister::BPL),
            "RSP" => Ok(PhysicalRegister::RSP),
            "ESP" => Ok(PhysicalRegister::ESP),
            "SP" => Ok(PhysicalRegister::SP),
            "SPL" => Ok(PhysicalRegister::SPL),
            "R8" => Ok(PhysicalRegister::R8),
            "R8D" => Ok(PhysicalRegister::R8D),
            "R8W" => Ok(PhysicalRegister::R8W),
            "R8B" => Ok(PhysicalRegister::R8B),
            "R9" => Ok(PhysicalRegister::R9),
            "R9D" => Ok(PhysicalRegister::R9D),
            "R9W" => Ok(PhysicalRegister::R9W),
            "R9B" => Ok(PhysicalRegister::R9B),
            "R10" => Ok(PhysicalRegister::R10),
            "R10D" => Ok(PhysicalRegister::R10D),
            "R10W" => Ok(PhysicalRegister::R10W),
            "R10B" => Ok(PhysicalRegister::R10B),
            "R11" => Ok(PhysicalRegister::R11),
            "R11D" => Ok(PhysicalRegister::R11D),
            "R11W" => Ok(PhysicalRegister::R11W),
            "R11B" => Ok(PhysicalRegister::R11B),
            "R12" => Ok(PhysicalRegister::R12),
            "R12D" => Ok(PhysicalRegister::R12D),
            "R12W" => Ok(PhysicalRegister::R12W),
            "R12B" => Ok(PhysicalRegister::R12B),
            "R13" => Ok(PhysicalRegister::R13),
            "R13D" => Ok(PhysicalRegister::R13D),
            "R13W" => Ok(PhysicalRegister::R13W),
            "R13B" => Ok(PhysicalRegister::R13B),
            "R14" => Ok(PhysicalRegister::R14),
            "R14D" => Ok(PhysicalRegister::R14D),
            "R14W" => Ok(PhysicalRegister::R14W),
            "R14B" => Ok(PhysicalRegister::R14B),
            "R15" => Ok(PhysicalRegister::R15),
            "R15D" => Ok(PhysicalRegister::R15D),
            "R15W" => Ok(PhysicalRegister::R15W),
            "R15B" => Ok(PhysicalRegister::R15B),
            _ => Err(()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();
            if self.peek() == Some('/') && self.peek_ahead(1) == Some('/') {
                self.skip_comment();
                continue;
            }
            break;
        }

        match self.peek() {
            None => Token::Eof,
            Some('(') => {
                self.advance();
                Token::LParen
            }
            Some(')') => {
                self.advance();
                Token::RParen
            }
            Some('{') => {
                self.advance();
                Token::LBrace
            }
            Some('}') => {
                self.advance();
                Token::RBrace
            }
            Some('[') => {
                self.advance();
                Token::LBracket
            }
            Some(']') => {
                self.advance();
                Token::RBracket
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some(':') => {
                self.advance();
                Token::Colon
            }
            Some('=') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::EqEq
                } else {
                    Token::Equal
                }
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                if self.peek().map_or(false, |c| c.is_ascii_digit()) {
                    // neg nr
                    self.pos -= 1;
                    Token::Integer(self.read_number())
                } else {
                    Token::Minus
                }
            }
            Some('*') => {
                self.advance();
                Token::Star
            }
            Some('!') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::NotEq
                } else {
                    // invalid is skip
                    Token::Eof
                }
            }
            Some('<') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            Some(ch) if ch.is_ascii_digit() => Token::Integer(self.read_number()),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_ident();
                self.ident_to_token(&ident)
            }
            Some(_) => {
                // skip wierd
                self.advance();
                self.next_token()
            }
        }
    }
}

/// another parser with inverse
pub struct Parser {
    tokens: Vec<(Token, usize)>,
    pos: usize,
    diagnostics: Vec<Diagnostic>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        let mut char_pos = 0;

        loop {
            let start_pos = lexer.current_pos();
            let token = lexer.next_token();
            if token == Token::Eof {
                tokens.push((token, start_pos));
                break;
            }
            tokens.push((token, start_pos));
        }

        Parser {
            tokens,
            pos: 0,
            diagnostics: Vec::new(),
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens.get(self.pos).map(|(t, _)| t).unwrap_or(&Token::Eof)
    }

    fn current_span(&self) -> usize {
        self.tokens
            .get(self.pos)
            .map(|(_, s)| *s)
            .unwrap_or(self.tokens.last().map(|(_, s)| *s).unwrap_or(0))
    }

    fn advance(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        self.current_token().clone()
    }

    fn expect(&mut self, expected: Token) -> Result<(), ()> {
        if self.current_token() == &expected {
            self.advance();
            Ok(())
        } else {
            let span = self.current_span();
            self.diagnostics.push(Diagnostic {
                message: format!(
                    "Expected {}, found {}",
                    expected, self.current_token()
                ),
                span: Some(Span { start: span, end: span + 1 }),
            });
            Err(())
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<Diagnostic>> {
        let mut functions = Vec::new();

        while self.current_token() != &Token::Eof {
            match self.parse_function() {
                Ok(func) => functions.push(func),
                Err(_) => {
                    // skip next function
                    while self.current_token() != &Token::Eof
                        && self.current_token() != &Token::Fn
                    {
                        self.advance();
                    }
                }
            }
        }

        if self.diagnostics.is_empty() {
            Ok(Program { functions })
        } else {
            Err(self.diagnostics.drain(..).collect())
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, ()> {
        self.expect(Token::Fn)?;

        let name = if let Token::Ident(n) = self.current_token() {
            let n = n.clone();
            self.advance();
            n
        } else {
            let span = self.current_span();
            self.diagnostics.push(Diagnostic {
                message: "Expected function name".to_string(),
                span: Some(Span { start: span, end: span + 1 }),
            });
            return Err(());
        };

        self.expect(Token::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;
        let body = self.parse_scope_block()?;
        self.expect(Token::RBrace)?;

        Ok(FunctionDecl { name, params, body })
    }

    fn parse_param_list(&mut self) -> Result<Vec<Ident>, ()> {
        let mut params = Vec::new();

        if self.current_token() == &Token::RParen {
            return Ok(params);
        }

        loop {
            if let Token::Ident(p) = self.current_token() {
                params.push(p.clone());
                self.advance();
            } else {
                let span = self.current_span();
                self.diagnostics.push(Diagnostic {
                    message: "Expected parameter name".to_string(),
                    span: Some(Span { start: span, end: span + 1 }),
                });
                return Err(());
            }

            if self.current_token() == &Token::Comma {
                self.advance();
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_scope_block(&mut self) -> Result<ScopeBlock, ()> {
        let mut statements = Vec::new();

        while self.current_token() != &Token::RBrace && self.current_token() != &Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                    // rec. next statement
                    while self.current_token() != &Token::RBrace
                        && self.current_token() != &Token::Eof
                        && !self.is_statement_start()
                    {
                        self.advance();
                    }
                }
            }
        }

        Ok(ScopeBlock { statements })
    }

    fn is_statement_start(&self) -> bool {
        matches!(
            self.current_token(),
            Token::Alias | Token::If | Token::While | Token::LBrace | Token::Ident(_)
        )
    }

    fn parse_statement(&mut self) -> Result<Statement, ()> {
        match self.current_token() {
            Token::Alias => self.parse_alias_decl().map(Statement::AliasDecl),
            Token::If => self.parse_if().map(Statement::If),
            Token::While => self.parse_while().map(Statement::While),
            Token::LBrace => self.parse_nested_block().map(Statement::ScopeBlock),
            Token::Ident(_) => self.parse_instruction().map(Statement::Instruction),
            _ => {
                let span = self.current_span();
                self.diagnostics.push(Diagnostic {
                    message: format!("Unexpected token: {}", self.current_token()),
                    span: Some(Span { start: span, end: span + 1 }),
                });
                Err(())
            }
        }
    }

    fn parse_alias_decl(&mut self) -> Result<AliasDecl, ()> {
        let start = self.current_span();
        self.expect(Token::Alias)?;

        let name = if let Token::Ident(n) = self.current_token() {
            let n = n.clone();
            self.advance();
            n
        } else {
            let span = self.current_span();
            self.diagnostics.push(Diagnostic {
                message: "Expected alias name".to_string(),
                span: Some(Span { start: span, end: span + 1 }),
            });
            return Err(());
        };

        self.expect(Token::Colon)?;

        let width = match self.current_token() {
            Token::Reg64 => {
                self.advance();
                RegisterWidth::Reg64
            }
            Token::Reg32 => {
                self.advance();
                RegisterWidth::Reg32
            }
            Token::Reg16 => {
                self.advance();
                RegisterWidth::Reg16
            }
            Token::Reg8 => {
                self.advance();
                RegisterWidth::Reg8
            }
            _ => {
                let span = self.current_span();
                self.diagnostics.push(Diagnostic {
                    message: "Expected register width (reg64, reg32, reg16, or reg8)"
                        .to_string(),
                    span: Some(Span { start: span, end: span + 1 }),
                });
                return Err(());
            }
        };

        self.expect(Token::Equal)?;

        let physical = if let Token::PhysReg(r) = self.current_token() {
            let r = *r;
            self.advance();
            r
        } else {
            let span = self.current_span();
            self.diagnostics.push(Diagnostic {
                message: "Expected physical register".to_string(),
                span: Some(Span { start: span, end: span + 1 }),
            });
            return Err(());
        };

        let end = self.current_span();
        Ok(AliasDecl {
            name,
            width,
            physical,
            span: Some(Span { start, end }),
        })
    }

    fn parse_instruction(&mut self) -> Result<Instruction, ()> {
        let mnemonic = if let Token::Ident(m) = self.current_token() {
            Mnemonic(m.clone())
        } else {
            let span = self.current_span();
            self.diagnostics.push(Diagnostic {
                message: "Expected mnemonic".to_string(),
                span: Some(Span { start: span, end: span + 1 }),
            });
            return Err(());
        };
        self.advance();

        let mut operands = Vec::new();

        // instr can have either 0/1/2
        if self.is_operand_start() {
            operands.push(self.parse_operand()?);

            if self.current_token() == &Token::Comma {
                self.advance();
                operands.push(self.parse_operand()?);
            }
        }

        Ok(Instruction { mnemonic, operands })
    }

    fn is_operand_start(&self) -> bool {
        matches!(
            self.current_token(),
            Token::Ident(_) | Token::PhysReg(_) | Token::Integer(_) | Token::LBracket | Token::Minus
        )
    }

    fn parse_operand(&mut self) -> Result<Operand, ()> {
        match self.current_token() {
            Token::Integer(_) | Token::Minus => {
                if let Token::Integer(i) = self.current_token() {
                    let i = *i;
                    self.advance();
                    Ok(Operand::Immediate(i))
                } else {
                    let span = self.current_span();
                    self.diagnostics.push(Diagnostic {
                        message: "Expected number after minus".to_string(),
                        span: Some(Span { start: span, end: span + 1 }),
                    });
                    Err(())
                }
            }
            Token::LBracket => self.parse_memory_address().map(Operand::MemoryAddress),
            Token::Ident(_) | Token::PhysReg(_) => {
                self.parse_register().map(Operand::Register)
            }
            _ => {
                let span = self.current_span();
                self.diagnostics.push(Diagnostic {
                    message: format!("Expected operand, found {}", self.current_token()),
                    span: Some(Span { start: span, end: span + 1 }),
                });
                Err(())
            }
        }
    }

    fn parse_register(&mut self) -> Result<Register, ()> {
        if let Token::PhysReg(r) = self.current_token() {
            let r = *r;
            self.advance();
            Ok(Register::Physical(r))
        } else if let Token::Ident(n) = self.current_token() {
            let n = n.clone();
            self.advance();
            Ok(Register::Alias(n))
        } else {
            let span = self.current_span();
            self.diagnostics.push(Diagnostic {
                message: "Expected register".to_string(),
                span: Some(Span { start: span, end: span + 1 }),
            });
            Err(())
        }
    }

    fn parse_memory_address(&mut self) -> Result<MemoryAddress, ()> {
        self.expect(Token::LBracket)?;

        let base = if self.is_operand_start() && self.current_token() != &Token::Plus {
            Some(self.parse_register()?)
        } else {
            None
        };

        let index = if self.current_token() == &Token::Plus {
            self.advance();
            if let Token::Ident(_) | Token::PhysReg(_) = self.current_token() {
                let register = self.parse_register()?;
                self.expect(Token::Star)?;
                if let Token::Integer(scale) = self.current_token() {
                    let scale = *scale as u8;
                    self.advance();
                    Some(IndexedRegister { register, scale })
                } else {
                    let span = self.current_span();
                    self.diagnostics.push(Diagnostic {
                        message: "Expected scale factor".to_string(),
                        span: Some(Span { start: span, end: span + 1 }),
                    });
                    return Err(());
                }
            } else {
                None
            }
        } else {
            None
        };

        let displacement = if self.current_token() == &Token::Plus {
            self.advance();
            if let Token::Integer(d) = self.current_token() {
                let d = *d;
                self.advance();
                d
            } else {
                let span = self.current_span();
                self.diagnostics.push(Diagnostic {
                    message: "Expected displacement".to_string(),
                    span: Some(Span { start: span, end: span + 1 }),
                });
                return Err(());
            }
        } else if self.current_token() == &Token::Minus {
            self.advance();
            if let Token::Integer(d) = self.current_token() {
                let d = *d;
                self.advance();
                -d
            } else {
                let span = self.current_span();
                self.diagnostics.push(Diagnostic {
                    message: "Expected displacement".to_string(),
                    span: Some(Span { start: span, end: span + 1 }),
                });
                return Err(());
            }
        } else {
            0
        };

        self.expect(Token::RBracket)?;
        Ok(MemoryAddress {
            base,
            index,
            displacement,
        })
    }

    fn parse_if(&mut self) -> Result<If, ()> {
        self.expect(Token::If)?;
        let condition = self.parse_condition()?;
        self.expect(Token::LBrace)?;
        let then_block = self.parse_scope_block()?;
        self.expect(Token::RBrace)?;

        let else_block = if self.current_token() == &Token::Else {
            self.advance();
            self.expect(Token::LBrace)?;
            let else_blk = self.parse_scope_block()?;
            self.expect(Token::RBrace)?;
            Some(else_blk)
        } else {
            None
        };

        Ok(If {
            condition,
            then_block,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<While, ()> {
        self.expect(Token::While)?;
        let condition = self.parse_condition()?;
        self.expect(Token::LBrace)?;
        let body = self.parse_scope_block()?;
        self.expect(Token::RBrace)?;

        Ok(While { condition, body })
    }

    fn parse_condition(&mut self) -> Result<Condition, ()> {
        let left = self.parse_operand()?;
        let op = self.parse_comparison_op()?;
        let right = self.parse_operand()?;

        Ok(Condition { left, op, right })
    }

    fn parse_comparison_op(&mut self) -> Result<ComparisonOp, ()> {
        let op = match self.current_token() {
            Token::EqEq => ComparisonOp::Eq,
            Token::NotEq => ComparisonOp::Ne,
            Token::Lt => ComparisonOp::Lt,
            Token::Gt => ComparisonOp::Gt,
            Token::LtEq => ComparisonOp::Le,
            Token::GtEq => ComparisonOp::Ge,
            _ => {
                let span = self.current_span();
                self.diagnostics.push(Diagnostic {
                    message: "Expected comparison operator".to_string(),
                    span: Some(Span { start: span, end: span + 1 }),
                });
                return Err(());
            }
        };
        self.advance();
        Ok(op)
    }

    fn parse_nested_block(&mut self) -> Result<ScopeBlock, ()> {
        self.expect(Token::LBrace)?;
        let block = self.parse_scope_block()?;
        self.expect(Token::RBrace)?;
        Ok(block)
    }
}

pub fn parse(input: &str) -> Result<Program, Vec<Diagnostic>> {
    let mut parser = Parser::new(input);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_tokenizes_keywords() {
        let mut lexer = Lexer::new("fn alias if else while");
        assert_eq!(lexer.next_token(), Token::Fn);
        assert_eq!(lexer.next_token(), Token::Alias);
        assert_eq!(lexer.next_token(), Token::If);
        assert_eq!(lexer.next_token(), Token::Else);
        assert_eq!(lexer.next_token(), Token::While);
    }

    #[test]
    fn lexer_tokenizes_identifiers() {
        let mut lexer = Lexer::new("myVar _test");
        if let Token::Ident(s) = lexer.next_token() {
            assert_eq!(s, "myVar");
        } else {
            panic!("Expected identifier");
        }
        if let Token::Ident(s) = lexer.next_token() {
            assert_eq!(s, "_test");
        } else {
            panic!("Expected identifier");
        }
    }

    #[test]
    fn lexer_tokenizes_physical_registers() {
        let mut lexer = Lexer::new("RAX EAX AX");
        assert!(matches!(lexer.next_token(), Token::PhysReg(PhysicalRegister::RAX)));
        assert!(matches!(lexer.next_token(), Token::PhysReg(PhysicalRegister::EAX)));
        assert!(matches!(lexer.next_token(), Token::PhysReg(PhysicalRegister::AX)));
    }

    #[test]
    fn parser_parses_simple_function() {
        let input = "fn main() { }";
        let result = parse(input);
        assert!(result.is_ok());
        let prog = result.unwrap();
        assert_eq!(prog.functions.len(), 1);
        assert_eq!(prog.functions[0].name, "main");
    }

    #[test]
    fn parser_parses_alias_declaration() {
        let input = "fn test() { alias myvar : reg64 = RAX }";
        let result = parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parser_reports_missing_brace() {
        let input = "fn main()";
        let result = parse(input);
        assert!(result.is_err());
        let diags = result.unwrap_err();
        assert!(!diags.is_empty());
    }

    #[test]
    fn parser_parses_instruction_with_operands() {
        let input = "fn test() { mov RAX RBX }";
        let result = parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parser_parses_memory_operand() {
        let input = "fn test() { mov RAX [RBP + 8] }";
        let result = parse(input);
        assert!(result.is_ok());
    }
}
