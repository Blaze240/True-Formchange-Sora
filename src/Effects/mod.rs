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

mod Aerials;
mod Grounded_Moves;
mod Special;
mod SpecialAir;

pub fn install() {
    Aerials::install();
    Grounded_Moves::install();
    Special::install();
    SpecialAir::install();
}
