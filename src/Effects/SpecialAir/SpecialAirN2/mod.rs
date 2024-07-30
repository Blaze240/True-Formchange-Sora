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

unsafe extern "C" fn effect_specialairn2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_mainbody"),
            false,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_blitz"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_element"), true);
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_guardian"),
            false,
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
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_ice_hold"),
            Hash40::new("haver"),
            0,
            10,
            -1,
            -90,
            0,
            0,
            1,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_ice_sword_flare"),
            Hash40::new("haver"),
            0,
            10,
            -1,
            -90,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_ice_shot"),
            Hash40::new("haver"),
            0,
            10,
            -1,
            -90,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 25.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_ice_impact"),
                Hash40::new("haver"),
                0,
                10,
                -1,
                -90,
                0,
                0,
                1,
                true,
            );
        } else {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(
                    agent,
                    Hash40::new("trail_ice_impact"),
                    Hash40::new("haver"),
                    0,
                    10,
                    -1,
                    -90,
                    180,
                    0,
                    1,
                    true,
                );
            }
        }
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd(
            "effect_specialairn2",
            effect_specialairn2,
            Priority::Low,
        )
        .install();
}
