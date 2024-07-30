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

mod SpecialAirN1Start;
mod SpecialAirN2;
mod SpecialAirN3;

mod SpecialAirHi;
mod SpecialAirLwStart;

mod SpecialAirSStart;

pub fn install() {
    SpecialAirHi::install();
    SpecialAirLwStart::install();
    SpecialAirSStart::install();
    SpecialAirN1Start::install();
    SpecialAirN2::install();
    SpecialAirN3::install();
}
