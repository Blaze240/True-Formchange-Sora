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
unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent){
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_mainbody"),
            false,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_blitz"), false);
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_element"),
            false,
        );
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_guardian"),
            true,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_second"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_strike"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_ultima"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_light"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_dark"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_double"), false);

        DEFAULT_FLAG = true;
        BLITZ_FLAG = false;
        ELEMENT_FLAG = false;
        GUARDIAN_FLAG = false;
        SECOND_FLAG = false;
        STRIKE_FLAG = false;
        ULTIMA_FLAG = false;
        LIGHT_FLAG = false;
        DARK_FLAG = false;
        DOUBLE_FLAG = false;
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("trail_counter_flash"), Hash40::new("top"), 0, 9, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..4 {
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    wait(agent.lua_state_agent, 2.0);
}
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low)
        .install();
}
