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

mod GuardOn;

mod EscapeB;
mod EscapeF;
mod EscapeN;
mod EscapeAir;
mod EscapeAirSlide;

pub fn install() {
    GuardOn::install();

    EscapeB::install();
    EscapeF::install();
    EscapeN::install();
    EscapeAir::install();
    EscapeAirSlide::install();
}
