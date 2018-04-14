// #![deny(warning)]

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

        macro_rules! arith {
            ($op:expr) => {{
                let (a_ptr, a_ir) = ir.a.eval(pc, self.size, &self.core);
                let (b_ptr, b_ir) = ir.b.eval(pc, self.size, &self.core);
                match ir.modifier {
                    Modifier::A => self.core.0[(pc + b_ptr) % self.size].a.number =
                        $op(b_ir.a.number, a_ir.a.number) % self.size,
                    Modifier::B => self.core.0[(pc + b_ptr) % self.size].a.number =
                        $op(b_ir.b.number, a_ir.b.number) % self.size,
                    Modifier::AB => self.core.0[(pc + b_ptr) % self.size].b.number =
                        $op(b_ir.a.number, a_ir.b.number) % self.size,
                    Modifier::BA => self.core.0[(pc + b_ptr) % self.size].a.number =
                        $op(b_ir.b.number, a_ir.a.number) % self.size,
                    Modifier::F | Modifier::I => {
                        self.core.0[(pc + b_ptr) % self.size].b.number =
                            $op(b_ir.a.number, a_ir.a.number) % self.size;
                        self.core.0[(pc + b_ptr) % self.size].a.number =
                            $op(b_ir.b.number, a_ir.b.number) % self.size;
                    }
                    Modifier::X => {
                        self.core.0[(pc + b_ptr) % self.size].b.number =
                            $op(b_ir.a.number, a_ir.b.number) % self.size;
                        self.core.0[(pc + b_ptr) % self.size].a.number =
                            $op(b_ir.b.number, a_ir.a.number) % self.size;
                    }
                }
                self.warrior.queue.push_back((pc + 1) % self.size)
            }}
        }

        macro_rules! arith_div {
            ($op:expr) => {{
                let mut no_queue = false;
                let (a_ptr, a_ir) = ir.a.eval(pc, self.size, &self.core);
                let (b_ptr, b_ir) = ir.b.eval(pc, self.size, &self.core);
                match ir.modifier {
                    Modifier::A => self.core.0[(pc + b_ptr) % self.size].a.number =
                        $op(b_ir.a.number, a_ir.a.number) % self.size,
                    Modifier::B => self.core.0[(pc + b_ptr) % self.size].a.number =
                        $op(b_ir.b.number, a_ir.b.number) % self.size,
                    Modifier::AB => self.core.0[(pc + b_ptr) % self.size].b.number =
                        $op(b_ir.b.number, a_ir.a.number) % self.size,
                    Modifier::BA => self.core.0[(pc + b_ptr) % self.size].a.number =
                        $op(b_ir.a.number, a_ir.b.number) % self.size,
                    Modifier::F | Modifier::I => {
                        if a_ir.a.number != 0 {
                            self.core.0[(pc + b_ptr) % self.size].a.number =
                                $op(b_ir.a.number, a_ir.a.number) % self.size;
                        }
                        if a_ir.b.number != 0 {
                            self.core.0[(pc + b_ptr) % self.size].b.number =
                                $op(b_ir.b.number, a_ir.b.number) % self.size;
                        }
                        no_queue = a_ir.a.number != 0 || a_ir.b.number != 0;
                    }
                    Modifier::X => {
                        if a_ir.a.number != 0 {
                            self.core.0[(pc + b_ptr) % self.size].b.number =
                                $op(b_ir.b.number, a_ir.a.number) % self.size;
                        }
                        if a_ir.b.number != 0 {
                            self.core.0[(pc + b_ptr) % self.size].a.number =
                                $op(b_ir.a.number, a_ir.b.number) % self.size;
                        }
                        no_queue = a_ir.a.number != 0 || a_ir.b.number != 0;
                    }
                }
                if !no_queue {
                    self.warrior.queue.push_back((pc + 1) % self.size)
                }
            }}
        }

        match ir.code {
            OpCode::DAT => (),
            OpCode::MOV => {
                let (a_ptr, a_ir) = ir.a.eval(pc, self.size, &self.core);
                let (b_ptr, b_ir) = ir.b.eval(pc, self.size, &self.core);
                match ir.modifier {
                    Modifier::A => self.core.0[(pc + b_ptr) % self.size].a.number = a_ir.a.number,
                    Modifier::B => self.core.0[(pc + b_ptr) % self.size].a.number = a_ir.b.number,
                    Modifier::AB => self.core.0[(pc + b_ptr) % self.size].b.number = a_ir.a.number,
                    Modifier::BA => self.core.0[(pc + b_ptr) % self.size].a.number = a_ir.b.number,
                    Modifier::F => {
                        self.core.0[(pc + b_ptr) % self.size].a.number = a_ir.a.number;
                        self.core.0[(pc + b_ptr) % self.size].b.number = a_ir.b.number;
                    }
                    Modifier::X => {
                        self.core.0[(pc + b_ptr) % self.size].b.number = a_ir.a.number;
                        self.core.0[(pc + b_ptr) % self.size].a.number = a_ir.b.number;
                    }
                    Modifier::I => {
                        self.core.0[(pc + b_ptr) % self.size] = a_ir;
                    }
                }
                self.warrior.queue.push_back((pc + 1) % self.size)
            }
            OpCode::ADD => arith!(|x, y| x + y),
            OpCode::SUB => arith!(|x, y| x + self.size - y),
            OpCode::MUL => arith!(|x, y| x * y),
            OpCode::DIV => arith_div!(|x, y| x / y),
            OpCode::MOD => arith_div!(|x, y| x % y),
            OpCode::JMP => {
                let (a_ptr, _) = ir.a.eval(pc, self.size, &self.core);
                self.warrior.queue.push_back(a_ptr);
            }
            OpCode::CMP => {
                let (_, a_ir) = ir.a.eval(pc, self.size, &self.core);
                let (_, b_ir) = ir.b.eval(pc, self.size, &self.core);
                match ir.modifier {
                    Modifier::A => {
                        if a_ir.a.number == b_ir.a.number {
                            self.warrior.queue.push_back((pc + 2) % self.size)
                        } else {
                            self.warrior.queue.push_back((pc + 1) % self.size)
                        }
                    }
                    Modifier::B => {
                        if a_ir.b.number == b_ir.b.number {
                            self.warrior.queue.push_back((pc + 2) % self.size)
                        } else {
                            self.warrior.queue.push_back((pc + 1) % self.size)
                        }
                    }
                    Modifier::AB => {
                        if a_ir.a.number == b_ir.b.number {
                            self.warrior.queue.push_back((pc + 2) % self.size)
                        } else {
                            self.warrior.queue.push_back((pc + 1) % self.size)
                        }
                    }
                    Modifier::BA => {
                        if a_ir.b.number == b_ir.a.number {
                            self.warrior.queue.push_back((pc + 2) % self.size)
                        } else {
                            self.warrior.queue.push_back((pc + 1) % self.size)
                        }
                    }
                    Modifier::F => {
                        if a_ir.a.number == b_ir.a.number
                                && a_ir.b.number == b_ir.b.number {
                            self.warrior.queue.push_back((pc + 2) % self.size)
                        } else {
                            self.warrior.queue.push_back((pc + 1) % self.size)
                        }
                    }
                    Modifier::X => {
                        if a_ir.a.number == b_ir.b.number
                            && a_ir.b.number == b_ir.a.number {
                            self.warrior.queue.push_back((pc + 2) % self.size)
                        } else {
                            self.warrior.queue.push_back((pc + 1) % self.size)
                        }
                    }
                    Modifier::I => {
                        if a_ir == b_ir {
                            self.warrior.queue.push_back((pc + 2) % self.size)
                        } else {
                            self.warrior.queue.push_back((pc + 1) % self.size)
                        }
                    }
                }
            }
            _ => unimplemented!(),
        }
        true
    }
}

struct Warrior {
    queue: VecDeque<usize>,
}

struct Core(Vec<Instruction>);

impl Core {
    fn print(&self) {
        for ir in &self.0 {
            println!("{}", ir);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Instruction {
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
enum Modifier {
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
