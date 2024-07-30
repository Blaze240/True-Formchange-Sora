#![feature(concat_idents, proc_macro_hygiene)]
#![allow(
    unused_imports,
    unused_macros,
    unused_variables,
    unused_assignments,
    unused_unsafe,
    non_upper_case_globals,
    non_snake_case,
    clippy::borrow_interior_mutable_const
)]
static mut DEFAULT_FLAG: bool = false;

static mut BLITZ_FLAG: bool = false;
static mut ELEMENT_FLAG: bool = false;
static mut GUARDIAN_FLAG: bool = false;
static mut SECOND_FLAG: bool = false;
static mut STRIKE_FLAG: bool = false;
static mut ULTIMA_FLAG: bool = false;
static mut LIGHT_FLAG: bool = false;
static mut DARK_FLAG: bool = false;
static mut DOUBLE_FLAG: bool = false;

mod Effects;
mod Ultima_Check;

#[skyline::main(name = "true_formchange")]
pub fn main() {
    Ultima_Check::install();
    Effects::install();
}
