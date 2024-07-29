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

unsafe extern "C" fn effect_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("null"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("trail_air_lw_impact_ultima"),
                Hash40::new("top"),
                0,
                0,
                0,
                0,
                180,
                0,
                1,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
            );
        } 
    }else {
            if macros::is_excute(agent) {
                macros::EFFECT(
                    agent,
                    Hash40::new("trail_air_lw_impact_ultima"),
                    Hash40::new("top"),
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                );
            }
        }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_landingairlw_ultima", effect_landingairlw, Priority::Low)
        .install();
}
