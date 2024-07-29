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

mod SpecialAirHi;
mod SpecialAirLw;
mod SpecialAirS1;
mod SpecialAirS2;
mod SpecialAirS3;
mod SpecialAirSStart;
mod SpecialAirSSearch;

pub fn install() {
    SpecialAirHi::install();
    SpecialAirLw::install();
    SpecialAirSStart::install();
    SpecialAirS1::install();
    SpecialAirS2::install();
    SpecialAirS3::install();
    SpecialAirSSearch::install();
}
