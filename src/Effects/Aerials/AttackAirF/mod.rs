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

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_keyblade_flare"),
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
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_atk_slash_air_f"),
            Hash40::new("top"),
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
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_attackairf", effect_attackairf, Priority::Low)
        .install();
}
