use Core;
use Instruction;
use Modifier;

pub fn mov(ir: Instruction, pc: usize, core: &mut Core, m: usize) -> Vec<usize> {
    let (a_ptr, a_ir) = ir.a.eval(pc, m, &core);
    let (b_ptr, b_ir) = ir.b.eval(pc, m, &core);
    match ir.modifier {
        Modifier::A => core.0[(pc + b_ptr) % m].a.number = a_ir.a.number,
        Modifier::B => core.0[(pc + b_ptr) % m].a.number = a_ir.b.number,
        Modifier::AB => core.0[(pc + b_ptr) % m].b.number = a_ir.a.number,
        Modifier::BA => core.0[(pc + b_ptr) % m].a.number = a_ir.b.number,
        Modifier::F => {
            core.0[(pc + b_ptr) % m].a.number = a_ir.a.number;
            core.0[(pc + b_ptr) % m].b.number = a_ir.b.number;
        }
        Modifier::X => {
            core.0[(pc + b_ptr) % m].b.number = a_ir.a.number;
            core.0[(pc + b_ptr) % m].a.number = a_ir.b.number;
        }
        Modifier::I => {
            core.0[(pc + b_ptr) % m] = a_ir;
        }
    }
    vec![(pc + 1) % m]
}
