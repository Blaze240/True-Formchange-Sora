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

static mut RAGE_FLAG: bool = false;

mod Form_Switch;

#[skyline::main(name = "true_rage_form")]
pub fn main() {
    Form_Switch::install();
}
