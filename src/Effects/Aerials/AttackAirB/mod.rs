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
unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_keyblade_flare_ultima"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 12.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_atk_slash_air_b_ultima"),
                Hash40::new("top"),
                0.8,
                -1,
                0.7,
                0,
                0,
                5,
                1.04,
                true,
            );
        }
    } else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_atk_slash_air_b_ultima"),
                Hash40::new("top"),
                0,
                -0.2,
                -0.1,
                0,
                0,
                0,
                1,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ultima"), false, true);
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_attackairb_ultima", effect_attackairb, Priority::Low)
        .install();
}
