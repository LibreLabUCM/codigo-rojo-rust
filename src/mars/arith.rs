macro_rules! fn_arith {
    ($name:ident, $op:expr) => {
        pub fn $name(ir: Instruction, pc: usize, core: &mut Core) -> Vec<usize> {
            let m = core.0.len();
            let (a_ptr, a_ir) = ir.a.eval(pc, &core);
            let (b_ptr, b_ir) = ir.b.eval(pc, &core);
            match ir.modifier {
                Modifier::A => core.0[(pc + b_ptr) % m].a.number =
                    $op(b_ir.a.number, a_ir.a.number, m) % m,
                Modifier::B => core.0[(pc + b_ptr) % m].b.number =
                    $op(b_ir.b.number, a_ir.b.number, m) % m,
                Modifier::AB => core.0[(pc + b_ptr) % m].b.number =
                    $op(b_ir.b.number, a_ir.a.number, m) % m,
                Modifier::BA => core.0[(pc + b_ptr) % m].a.number =
                    $op(b_ir.a.number, a_ir.b.number, m) % m,
                Modifier::F | Modifier::I => {
                    core.0[(pc + b_ptr) % m].b.number =
                        $op(b_ir.a.number, a_ir.a.number, m) % m;
                    core.0[(pc + b_ptr) % m].a.number =
                        $op(b_ir.b.number, a_ir.b.number, m) % m;
                }
                Modifier::X => {
                    core.0[(pc + b_ptr) % m].b.number =
                        $op(b_ir.a.number, a_ir.b.number, m) % m;
                    core.0[(pc + b_ptr) % m].a.number =
                        $op(b_ir.b.number, a_ir.a.number, m) % m;
                }
            }
            vec![(pc + 1) % m]
        }
    }
}
