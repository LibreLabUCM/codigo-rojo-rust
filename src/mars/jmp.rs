use Core;
use Instruction;

pub fn jmp(ir: Instruction, pc: usize, core: &mut Core, m: usize) -> Vec<usize> {
    let (a_ptr, _) = ir.a.eval(pc, m, &core);
    vec![(pc + a_ptr) % m]
}
