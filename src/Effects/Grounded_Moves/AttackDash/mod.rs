use {
    crate::BLITZ_FLAG,
    crate::DARK_FLAG,
    crate::DEFAULT_FLAG,
    crate::DOUBLE_FLAG,
    crate::ELEMENT_FLAG,
    crate::GUARDIAN_FLAG,
    crate::LIGHT_FLAG,
    crate::SECOND_FLAG,
    crate::STRIKE_FLAG,
    crate::ULTIMA_FLAG,
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

unsafe extern "C" fn effect_attackdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_mainbody"),
            false,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_blitz"), true);
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_element"),
            false,
        );
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_guardian"),
            false,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_second"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_strike"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_ultima"), false);

        DEFAULT_FLAG = false;
        BLITZ_FLAG = true;
        ELEMENT_FLAG = false;
        GUARDIAN_FLAG = false;
        SECOND_FLAG = false;
        STRIKE_FLAG = false;
        ULTIMA_FLAG = false;
        LIGHT_FLAG = false;
        DARK_FLAG = false;
        DOUBLE_FLAG = false;
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_attackdash", effect_attackdash, Low)
        .install();
}
