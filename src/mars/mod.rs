mod mov;
pub use self::mov::mov;

#[macro_use]
mod arith;
use Core;
use Instruction;
use Modifier;
fn_arith!(add, |x, y, _| x + y);
fn_arith!(sub, |x, y, m| x + m - y);
fn_arith!(mul, |x, y, _| x * y);

#[macro_use]
mod arith_div;
fn_arith_div!(div, |x, y| x / y);
fn_arith_div!(mod_, |x, y| x % y);

mod cmp;
pub use self::cmp::cmp;

mod jmp;
pub use self::jmp::jmp;
