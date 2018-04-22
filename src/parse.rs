use std::str::Chars;
use std::iter::Peekable;

use Instruction;
use Modifier;
use OpCode;
use Operand;
use AddressMode;

pub fn parse(s: &str) -> Option<Vec<Instruction>> {
    None
}

pub fn parse_ir(string: &str, core_size: usize) -> Option<Instruction> {
    let mut chars = string.chars().peekable();
    let code = match chars.by_ref().skip_while(|&c| c == ' ').take(3).collect::<String>().as_str() {
        "DAT" => OpCode::DAT,
        "MOV" => OpCode::MOV,
        "ADD" => OpCode::ADD,
        "SUB" => OpCode::SUB,
        "MUL" => OpCode::MUL,
        "DIV" => OpCode::DIV,
        "MOD" => OpCode::MOD,
        "JMP" => OpCode::JMP,
        "CMP" => OpCode::CMP,
        _ => return None,
    };
    let modifier = match chars.by_ref().take(2).collect::<String>().as_str() {
        ".A" => match chars.next() {
            Some('B') => Modifier::AB,
            Some(' ') => Modifier::A,
            _ => return None,
        },
        ".B" => match chars.next() {
            Some('A') => Modifier::BA,
            Some(' ') => Modifier::B,
            _ => return None,
        },
        ".F" => Modifier::F,
        ".X" => Modifier::X,
        ".I" => Modifier::I,
        _ => return None,
    };
    while let Some(&' ') = chars.peek() {
        chars.next();
    }
    let a = match parse_operand(chars.by_ref(), core_size) {
        Some(x) => x,
        None => return None,
    };
    let b;
    if code != OpCode::JMP {
        while let Some(&' ') = chars.peek() {
            chars.next();
        }
        if chars.next() != Some(',') {
            return None
        }
        b = match parse_operand(chars.by_ref(), core_size) {
            Some(x) => x,
            None => return None,
        };
    } else {
        b = Operand {
            mode: AddressMode::Immediate,
            number: 0,
        };
    }
    Some(Instruction {
        code,
        modifier,
        a,
        b,
    })
}

fn parse_operand(chars: &mut Peekable<Chars>, core_size: usize) -> Option<Operand> {
    let c = chars.by_ref().skip_while(|&c| c == ' ').next();
    let mut number_string = String::new();
    let mode = match c {
        Some('#') => AddressMode::Immediate,
        Some('$') => AddressMode::Direct,
        Some('@') => AddressMode::Indirect,
        Some('<') => AddressMode::PredecrementIndirect,
        Some('>') => AddressMode::PostincrementIndirect,
        Some(d) if d.is_digit(10) || d == '-' => {
            number_string.push(d);
            AddressMode::Direct
        },
        _ => return None,
    };
    if number_string.is_empty() && chars.peek() == Some(&'-') {
        number_string.push('-');
        chars.next();
    }
    loop {
        if let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                number_string.push(c);
                chars.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let number = match number_string.parse::<isize>() {
        Ok(x) if x < 0 => (core_size as isize + x) as usize % core_size,
        Ok(x) => x as usize % core_size,
        _ => return None,
    };
    Some(Operand {
        mode,
        number,
    })
}
