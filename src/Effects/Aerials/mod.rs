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

mod AttackAirN;
mod AttackAirF;
mod AttackAirB;
mod AttackAirHi;
mod AttackAirLw;
mod AttackAirF2;
mod AttackAirF3;
mod AttackAirN2;
mod AttackAirN3;

pub fn install() {
    AttackAirN::install();
    AttackAirLw::install();
    AttackAirB::install();
    AttackAirHi::install();
    AttackAirF::install();
    AttackAirF2::install();
    AttackAirF3::install();
    AttackAirN2::install();
    AttackAirN3::install();
}
