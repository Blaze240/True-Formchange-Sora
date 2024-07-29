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

mod Attack11;
mod Attack12;
mod Attack13;
mod AttackHi3;
mod AttackHi4;
mod AttackLw3;
mod AttackLw4;
mod AttackS3;
mod AttackS32;
mod AttackS33;
mod AttackS4;
mod ThrowF;
mod Landing_AirLw;

pub fn install() {
    Attack11::install();
    Attack12::install();
    Attack13::install();
    AttackS3::install();
    AttackS32::install();
    AttackS33::install();
    AttackHi3::install();
    AttackS4::install();
    AttackHi4::install();
    AttackLw3::install();
    AttackLw4::install();
    ThrowF::install();
    Landing_AirLw::install();
}
