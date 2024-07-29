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

mod SpecialHi;
mod SpecialLw;
mod SpecialS1;
mod SpecialS2;
mod SpecialS3;
mod SpecialSStart;
mod SpecialSSearch;

pub fn install() {
    SpecialHi::install();
    SpecialLw::install();
    SpecialSStart::install();
    SpecialS1::install();
    SpecialS2::install();
    SpecialS3::install();
    SpecialSSearch::install();
}
