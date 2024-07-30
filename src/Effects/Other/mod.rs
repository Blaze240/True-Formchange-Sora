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

mod Defense;
mod Entry;
mod Taunts;
mod Wait1;

pub fn install() {
    Wait1::install();
    Defense::install();
    Taunts::install();
    Entry::install();
}
