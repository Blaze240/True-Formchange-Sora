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
unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(
        agent.module_accessor,
        *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT,
    ) {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_counter_flash"),
                Hash40::new("top"),
                0,
                13,
                -1,
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
    frame(agent.lua_state_agent, 2.0);
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
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_trail_keyblade1"),
            Hash40::new("tex_trail_keyblade2"),
            14,
            Hash40::new("haver"),
            0,
            2,
            0,
            Hash40::new("haver"),
            0,
            13.8,
            0,
            true,
            Hash40::new("null"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.4,
            0.2,
        );
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_counter_attack"),
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
        macros::AFTER_IMAGE_OFF(agent, 3);
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_trail_keyblade1"),
            Hash40::new("tex_trail_keyblade2"),
            14,
            Hash40::new("haver"),
            0,
            2,
            0.5,
            Hash40::new("haver"),
            0,
            19,
            1,
            true,
            Hash40::new("null"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.4,
            0.2,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("trail_keyblade_light"),
            Hash40::new("haver"),
            0,
            5,
            0,
            0,
            0,
            0,
            1.25,
            true,
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 0);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_counter_attack"), -1);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_specialairlw", effect_specialairlw, Priority::Low)
        .install();
}
