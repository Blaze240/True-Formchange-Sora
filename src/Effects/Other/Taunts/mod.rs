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
mod Wait1;

pub fn install() {
    //Entry::install();

    // AppealHi::install();
    // AppealS::install();
    //AppealLw::install();

    Wait1::install();
    GuardOn::install();

    // Win1::install();
    // Win2::install();
    //Win3::install();
}
