// #![deny(warning)]

mod mars;
mod parse;

use std::collections::VecDeque;

fn main() {
    let mut program = Program {
        core: vec![],
        warrior: VecDeque::new(),
    };
    program.core.push(parse::parse_ir("MOV.I 0, 1").unwrap());
    for _ in 0..3 {
        program.core.push(parse::parse_ir("DAT.F #0, #0").unwrap());
    }
    program.warrior.push_back(0);
    print_core(&program.core);
    println!();
    for i in 0..25 {
        println!("{}", program.warrior[0]);
        assert!(program.next());
        print_core(&program.core);
        println!();
    }
}

struct Program {
    core: Core,
    warrior: VecDeque<usize>,
}

impl Program {
    fn next(&mut self) -> bool {
        let pc = match self.warrior.pop_front() {
            Some(pc) => pc,
            None => return false,
        };
        let ir = self.core[pc];
        use OpCode::*;
        use mars::{mov, add, sub, mul, div, mod_, jmp, cmp};
        let push_to_queue = match ir.code {
            DAT => vec![],
            MOV => mov(ir, pc, &mut self.core),
            ADD => add(ir, pc, &mut self.core),
            SUB => sub(ir, pc, &mut self.core),
            MUL => mul(ir, pc, &mut self.core),
            DIV => div(ir, pc, &mut self.core),
            MOD => mod_(ir, pc, &mut self.core),
            JMP => jmp(ir, pc, &mut self.core),
            CMP => cmp(ir, pc, &mut self.core),
        };
        for ptr in push_to_queue {
            self.warrior.push_back(ptr);
        }
        true
    }
}

type Core = Vec<Instruction>;

pub fn print_core(core: &Core) {
    for ir in core {
        println!("{}", ir);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instruction {
    code: OpCode,
    modifier: Modifier,
    a: Operand,
    b: Operand,
}

impl ::std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}.{} {}, {}", self.code, self.modifier, self.a, self.b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum OpCode {
    DAT,
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    JMP,
    //JMZ,
    //JMN,
    //DJN,
    CMP,
    //SLT,
    //SPL,
}

impl ::std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            OpCode::DAT => write!(f, "DAT"),
            OpCode::MOV => write!(f, "MOV"),
            OpCode::ADD => write!(f, "ADD"),
            OpCode::SUB => write!(f, "SUB"),
            OpCode::MUL => write!(f, "MUL"),
            OpCode::DIV => write!(f, "DIV"),
            OpCode::MOD => write!(f, "MOD"),
            OpCode::JMP => write!(f, "JMP"),
            OpCode::CMP => write!(f, "CMP"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

impl ::std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Modifier::A => write!(f, "A"),
            Modifier::B => write!(f, "B"),
            Modifier::AB => write!(f, "AB"),
            Modifier::BA => write!(f, "BA"),
            Modifier::F => write!(f, "F"),
            Modifier::X => write!(f, "X"),
            Modifier::I => write!(f, "I"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Operand {
    mode: AddressMode,
    number: usize,
}

impl Operand {
    fn eval(self, pc: usize, core: &Core) -> (usize, Instruction) {
        let m = core.len();
        let ptr: usize = match self.mode {
            AddressMode::Immediate => 0,
            AddressMode::Direct => self.number,
            AddressMode::Indirect =>
                self.number + core[(pc + self.number) % m].b.number,
            AddressMode::PredecrementIndirect =>
                self.number + core[(pc + self.number) % m].b.number + m - 1,
            AddressMode::PostincrementIndirect => unimplemented!(),
        };
        (ptr, core[(pc + ptr) % m])
    }
}

impl ::std::fmt::Display for Operand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}{}", self.mode, self.number)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum AddressMode {
    Immediate,
    Direct,
    Indirect,
    PredecrementIndirect,
    PostincrementIndirect,
}

impl ::std::fmt::Display for AddressMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AddressMode::Immediate => write!(f, "#"),
            AddressMode::Direct => write!(f, "$"),
            AddressMode::Indirect => write!(f, "@"),
            AddressMode::PredecrementIndirect => write!(f, "<"),
            AddressMode::PostincrementIndirect => write!(f, ">"),
        }
    }
}
