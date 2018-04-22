use Core;
use Instruction;

pub fn jmp(ir: Instruction, pc: usize, core: &mut Core) -> Vec<usize> {
    let m = core.0.len();
    let (a_ptr, _) = ir.a.eval(pc, &core);
    vec![(pc + a_ptr) % m]
}
