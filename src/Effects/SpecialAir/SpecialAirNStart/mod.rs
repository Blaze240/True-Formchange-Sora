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

unsafe extern "C" fn effect_specialairn1start(agent: &mut L2CAgentBase) {
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

        macros::EFFECT_FOLLOW_NO_STOP(
            agent,
            Hash40::new("trail_fire_hold"),
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
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd(
            "effect_specialairn1start",
            effect_specialairn1start,
            Priority::Low,
        )
        .install();
}
