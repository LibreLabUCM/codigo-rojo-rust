use Core;
use Instruction;
use Modifier;

pub fn cmp(ir: Instruction, pc: usize, core: &mut Core) -> Vec<usize> {
    let m = core.len();
    let (_, a_ir) = ir.a.eval(pc, &core);
    let (_, b_ir) = ir.b.eval(pc, &core);
    let two_or_one = match ir.modifier {
        Modifier::A => a_ir.a.number == b_ir.a.number,
        Modifier::B => a_ir.b.number == b_ir.b.number,
        Modifier::AB => a_ir.a.number == b_ir.b.number,
        Modifier::BA => a_ir.b.number == b_ir.a.number,
        Modifier::F => a_ir.a.number == b_ir.a.number && a_ir.b.number == b_ir.b.number,
        Modifier::X => a_ir.a.number == b_ir.b.number && a_ir.b.number == b_ir.a.number,
        Modifier::I => a_ir == b_ir,
    };
    if two_or_one {
        vec![(pc + 2) % m]
    } else {
        vec![(pc + 1) % m]
    }
}
