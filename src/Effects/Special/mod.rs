use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
};
mod SpecialN1Start;
mod SpecialN2;
mod SpecialN3;

mod SpecialHi;
mod SpecialLwStart;

mod SpecialSStart;

pub fn install() {
    SpecialN1Start::install();
    SpecialN2::install();
    SpecialN3::install();

    SpecialHi::install();
    SpecialLwStart::install();
    SpecialSStart::install();
}
