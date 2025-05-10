// FIXME: use mrubyedge?

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub op: OpCode,
    pub operand: Operand,
}

impl Bytecode {
    pub fn new(op: OpCode, operand: Operand) -> Self {
        Bytecode { op, operand }
    }

    pub fn to_bytes_vec(&self) -> Vec<u8> {
        let mut bytes = vec![self.op as u8];
        match self.operand {
            Operand::Z => {}
            Operand::B(b) => bytes.push(b),
            Operand::BB(b1, b2) => {
                bytes.push(b1);
                bytes.push(b2);
            }
            Operand::BBB(b1, b2, b3) => {
                bytes.push(b1);
                bytes.push(b2);
                bytes.push(b3);
            }
            Operand::BS(b1, s) => {
                bytes.push(b1);
                bytes.extend_from_slice(&s.to_be_bytes());
            }
            Operand::BSS(b1, s1, s2) => {
                bytes.push(b1);
                bytes.extend_from_slice(&s1.to_be_bytes());
                bytes.extend_from_slice(&s2.to_be_bytes());
            }
            Operand::S(s) => {
                bytes.extend_from_slice(&s.to_be_bytes());
            }
            Operand::W(w) => {
                // should be truncated into u24...
                let operand: [u8; 4] = w.to_be_bytes();
                bytes.extend_from_slice(&operand[1..=3]);
            }
        }
        bytes
    }
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    NOP,
    MOVE,
    LOADL,
    LOADI,
    LOADINEG,
    LOADI__1,
    LOADI_0,
    LOADI_1,
    LOADI_2,
    LOADI_3,
    LOADI_4,
    LOADI_5,
    LOADI_6,
    LOADI_7,
    LOADI16,
    LOADI32,
    LOADSYM,
    LOADNIL,
    LOADSELF,
    LOADT,
    LOADF,
    GETGV,
    SETGV,
    GETSV,
    SETSV,
    GETIV,
    SETIV,
    GETCV,
    SETCV,
    GETCONST,
    SETCONST,
    GETMCNST,
    SETMCNST,
    GETUPVAR,
    SETUPVAR,
    GETIDX,
    SETIDX,
    JMP,
    JMPIF,
    JMPNOT,
    JMPNIL,
    JMPUW,
    EXCEPT,
    RESCUE,
    RAISEIF,
    SSEND,
    SSENDB,
    SEND,
    SENDB,
    CALL,
    SUPER,
    ARGARY,
    ENTER,
    KEY_P,
    KEYEND,
    KARG,
    RETURN,
    RETURN_BLK,
    BREAK,
    BLKPUSH,
    ADD,
    ADDI,
    SUB,
    SUBI,
    MUL,
    DIV,
    EQ,
    LT,
    LE,
    GT,
    GE,
    ARRAY,
    ARRAY2,
    ARYCAT,
    ARYPUSH,
    ARYSPLAT,
    AREF,
    ASET,
    APOST,
    INTERN,
    SYMBOL,
    STRING,
    STRCAT,
    HASH,
    HASHADD,
    HASHCAT,
    LAMBDA,
    BLOCK,
    METHOD,
    RANGE_INC,
    RANGE_EXC,
    OCLASS,
    CLASS,
    MODULE,
    EXEC,
    DEF,
    ALIAS,
    UNDEF,
    SCLASS,
    TCLASS,
    DEBUG,
    ERR,
    EXT1,
    EXT2,
    EXT3,
    STOP,

    NumberOfOpcode, // for fetcher table
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Z,
    B(u8),
    BB(u8, u8),
    BBB(u8, u8, u8),
    BS(u8, u16),
    BSS(u8, u16, u16),
    S(u16),
    W(u32), // u24 in real layout
}