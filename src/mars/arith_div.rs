macro_rules! fn_arith_div {
    ($name:ident, $op:expr) => {
        pub fn $name(ir: Instruction, pc: usize, core: &mut Core) -> Vec<usize> {
            let m = core.len();
            let mut no_queue = false;
            let (a_ptr, a_ir) = ir.a.eval(pc, &core);
            let (b_ptr, b_ir) = ir.b.eval(pc, &core);
            match ir.modifier {
                Modifier::A => core[(pc + b_ptr) % m].a.number =
                    $op(b_ir.a.number, a_ir.a.number) % m,
                Modifier::B => core[(pc + b_ptr) % m].a.number =
                    $op(b_ir.b.number, a_ir.b.number) % m,
                Modifier::AB => core[(pc + b_ptr) % m].b.number =
                    $op(b_ir.b.number, a_ir.a.number) % m,
                Modifier::BA => core[(pc + b_ptr) % m].a.number =
                    $op(b_ir.a.number, a_ir.b.number) % m,
                Modifier::F | Modifier::I => {
                    if a_ir.a.number != 0 {
                        core[(pc + b_ptr) % m].a.number =
                            $op(b_ir.a.number, a_ir.a.number) % m;
                    }
                    if a_ir.b.number != 0 {
                        core[(pc + b_ptr) % m].b.number =
                            $op(b_ir.b.number, a_ir.b.number) % m;
                    }
                    no_queue = a_ir.a.number != 0 || a_ir.b.number != 0;
                }
                Modifier::X => {
                    if a_ir.a.number != 0 {
                        core[(pc + b_ptr) % m].b.number =
                            $op(b_ir.b.number, a_ir.a.number) % m;
                    }
                    if a_ir.b.number != 0 {
                        core[(pc + b_ptr) % m].a.number =
                            $op(b_ir.a.number, a_ir.b.number) % m;
                    }
                    no_queue = a_ir.a.number != 0 || a_ir.b.number != 0;
                }
            }
            if no_queue {
                vec![]
            } else {
                vec![(pc + 1) % m]
            }
        }
    }
}
