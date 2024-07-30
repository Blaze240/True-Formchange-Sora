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

unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_mainbody"),
            false,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_blitz"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_element"), false);
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("trail_guardian"),
            false,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_second"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_strike"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_ultima"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_light"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_dark"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("trail_double"), false);

        DEFAULT_FLAG = false;
        BLITZ_FLAG = false;
        ELEMENT_FLAG = false;
        GUARDIAN_FLAG = false;
        SECOND_FLAG = false;
        STRIKE_FLAG = true;
        ULTIMA_FLAG = false;
        LIGHT_FLAG = false;
        DARK_FLAG = false;
        DOUBLE_FLAG = false;
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("haver"),
            0,
            10,
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
            true,
        );
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_smash_hi_keyblade"),
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
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_smash_hi_speedline"),
            Hash40::new("top"),
            0,
            10,
            10,
            0,
            0,
            0,
            1,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(
            agent,
            Hash40::new("trail_smash_hi_flash2"),
            Hash40::new("haver"),
            0,
            10,
            -0.5,
            0,
            0,
            0,
            1.2,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_smash_hi_flash"),
            Hash40::new("haver"),
            0,
            10,
            -0.5,
            0,
            0,
            0,
            1.2,
            true,
        );
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_v_smoke_a"),
            Hash40::new("top"),
            1,
            0,
            0,
            0,
            0,
            0,
            0.7,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash2"), -1);
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_attackhi4", effect_attackhi4, Priority::Low)
        .install();
}
