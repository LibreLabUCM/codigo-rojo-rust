// #![deny(warning)]

use std::collections::VecDeque;

fn main() {
    let mut program = Program {
        core: vec![],
        size: 0,
        warrior: Warrior {
            queue: VecDeque::new(),
        }
    };
    program.core.push(Instruction {
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
    for _ in 0..9 {
        program.core.push(Instruction {
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
    println!("{:?}\n", program.core);
    for i in 0..2 {
        assert!(program.next());
        println!("{:?}\n", program.core);
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
        let ir = self.core[pc];
        match ir.code {
            OpCode::DAT => (),
            OpCode::MOV => {
                let (a_ptr, a_ir) = ir.a.eval(pc, self.size, &self.core);
                let (b_ptr, b_ir) = ir.b.eval(pc, self.size, &self.core);
                println!("{:?}", ir.modifier);
                println!("{:?}", b_ptr);
                match ir.modifier {
                    Modifier::A => self.core[(pc + b_ptr) % self.size].a.number = a_ir.a.number,
                    Modifier::B => self.core[(pc + b_ptr) % self.size].a.number = a_ir.b.number,
                    Modifier::AB => self.core[(pc + b_ptr) % self.size].b.number = a_ir.a.number,
                    Modifier::BA => self.core[(pc + b_ptr) % self.size].a.number = a_ir.b.number,
                    Modifier::F => {
                        self.core[(pc + b_ptr) % self.size].a.number = a_ir.a.number;
                        self.core[(pc + b_ptr) % self.size].b.number = a_ir.b.number;
                    }
                    Modifier::X => {
                        self.core[(pc + b_ptr) % self.size].b.number = a_ir.a.number;
                        self.core[(pc + b_ptr) % self.size].a.number = a_ir.b.number;
                    }
                    Modifier::I => {
                        self.core[(pc + b_ptr) % self.size] = a_ir;
                    }
                }
                self.warrior.queue.push_back((pc + 1) % self.size)
            }
            _ => unimplemented!(),
        }
        true
    }
}

struct Warrior {
    queue: VecDeque<usize>,
}

type Core = Vec<Instruction>;

/*
impl ::std::fmt::Debug for Core {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
*/

#[derive(Clone, Copy, Debug)]
struct Instruction {
    code: OpCode,
    modifier: Modifier,
    a: Operand,
    b: Operand,
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    DAT,
    MOV,
    ADD,
    //SUB,
    //MUL,
    //DIV,
    //MOD,
    JMP,
    //JMZ,
    //JMN,
    //DJN,
    CMP,
    //SLT,
    //SPL,
}

#[derive(Clone, Copy, Debug)]
enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

#[derive(Clone, Copy, Debug)]
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
                self.number + core[(pc + self.number) % m].b.number,
            AddressMode::PredecrementIndirect =>
                self.number + core[(pc + self.number) % m].b.number + m - 1,
            AddressMode::PostincrementIndirect => unimplemented!(),
        };
        (ptr, core[pc + ptr % m])
    }
}

#[derive(Clone, Copy, Debug)]
enum AddressMode {
    Immediate,
    Direct,
    Indirect,
    PredecrementIndirect,
    PostincrementIndirect,
}
