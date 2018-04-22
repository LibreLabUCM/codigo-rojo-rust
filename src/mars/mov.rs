use Core;
use Instruction;
use Modifier;

pub fn mov(ir: Instruction, pc: usize, core: &mut Core) -> Vec<usize> {
    let m = core.0.len();
    let (a_ptr, a_ir) = ir.a.eval(pc, &core);
    let (b_ptr, b_ir) = ir.b.eval(pc, &core);
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
