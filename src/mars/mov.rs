use Core;
use Instruction;
use Modifier;

pub fn mov(ir: Instruction, pc: usize, core: &mut Core) -> Vec<usize> {
    let m = core.len();
    let (a_ptr, a_ir) = ir.a.eval(pc, &core);
    let (b_ptr, b_ir) = ir.b.eval(pc, &core);
    match ir.modifier {
        Modifier::A => core[(pc + b_ptr) % m].a.number = a_ir.a.number,
        Modifier::B => core[(pc + b_ptr) % m].a.number = a_ir.b.number,
        Modifier::AB => core[(pc + b_ptr) % m].b.number = a_ir.a.number,
        Modifier::BA => core[(pc + b_ptr) % m].a.number = a_ir.b.number,
        Modifier::F => {
            core[(pc + b_ptr) % m].a.number = a_ir.a.number;
            core[(pc + b_ptr) % m].b.number = a_ir.b.number;
        }
        Modifier::X => {
            core[(pc + b_ptr) % m].b.number = a_ir.a.number;
            core[(pc + b_ptr) % m].a.number = a_ir.b.number;
        }
        Modifier::I => {
            core[(pc + b_ptr) % m] = a_ir;
        }
    }
    vec![(pc + 1) % m]
}
