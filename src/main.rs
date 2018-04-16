// #![deny(warning)]

mod mars;

use std::collections::VecDeque;

fn main() {
    let mut program = Program {
        core: Core(vec![]),
        size: 0,
        warrior: Warrior {
            queue: VecDeque::new(),
        }
    };
    program.core.0.push(Instruction {
        code: OpCode::MOV,
        modifier: Modifier::I,
        a: Operand {
            mode: AddressMode::Direct,
            number: 0,
        },
        b: Operand {
            mode: AddressMode::Direct,
            number: 1,
        },
    });
    program.size += 1;
    for _ in 0..3 {
        program.core.0.push(Instruction {
            code: OpCode::DAT,
            modifier: Modifier::F,
            a: Operand {
                mode: AddressMode::Immediate,
                number: 0,
            },
            b: Operand {
                mode: AddressMode::Immediate,
                number: 0,
            },
        });
        program.size += 1;
    }
    program.warrior.queue.push_back(0);
    program.core.print();
    println!();
    for i in 0..25 {
        println!("{}", program.warrior.queue[0]);
        assert!(program.next());
        program.core.print();
        println!();
    }
}

struct Program {
    core: Core,
    size: usize,
    warrior: Warrior
}

impl Program {
    fn next(&mut self) -> bool {
        let pc = match self.warrior.queue.pop_front() {
            Some(pc) => pc,
            None => return false,
        };
        let ir = self.core.0[pc];
        use OpCode::*;
        use mars::{mov, add, sub, mul, div, mod_, jmp, cmp};
        let push_to_queue = match ir.code {
            DAT => vec![],
            MOV => mov(ir, pc, &mut self.core, self.size),
            ADD => add(ir, pc, &mut self.core, self.size),
            SUB => sub(ir, pc, &mut self.core, self.size),
            MUL => mul(ir, pc, &mut self.core, self.size),
            DIV => div(ir, pc, &mut self.core, self.size),
            MOD => mod_(ir, pc, &mut self.core, self.size),
            JMP => jmp(ir, pc, &mut self.core, self.size),
            CMP => cmp(ir, pc, &mut self.core, self.size),
        };
        for ptr in push_to_queue {
            self.warrior.queue.push_back(ptr);
        }
        true
    }
}

struct Warrior {
    queue: VecDeque<usize>,
}

pub struct Core(Vec<Instruction>);

impl Core {
    fn print(&self) {
        for ir in &self.0 {
            println!("{}", ir);
        }
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
struct Operand {
    mode: AddressMode,
    number: usize,
}

impl Operand {
    fn eval(self, pc: usize, m: usize, core: &Core) -> (usize, Instruction) {
        let ptr: usize = match self.mode {
            AddressMode::Immediate => 0,
            AddressMode::Direct => self.number,
            AddressMode::Indirect =>
                self.number + core.0[(pc + self.number) % m].b.number,
            AddressMode::PredecrementIndirect =>
                self.number + core.0[(pc + self.number) % m].b.number + m - 1,
            AddressMode::PostincrementIndirect => unimplemented!(),
        };
        (ptr, core.0[(pc + ptr) % m])
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
