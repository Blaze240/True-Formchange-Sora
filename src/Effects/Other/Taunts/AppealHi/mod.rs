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

unsafe extern "C" fn effect_appealhi(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND,
    ) == 1
    {
        if macros::is_excute(agent) {
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_mainbody"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_blitz"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_element"),
                true,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_guardian"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_second"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_strike"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_ultima"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_light"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_dark"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_double"),
                false,
            );

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
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_cure_hold"),
                Hash40::new("haver"),
                0,
                12,
                0,
                0,
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
                Hash40::new("trail_cure_flower_root"),
                Hash40::new("top"),
                0,
                30,
                0,
                0,
                0,
                0,
                1,
                true,
            );
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(
                agent,
                Hash40::new("sys_v_smoke_a"),
                Hash40::new("top"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.65,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
            );
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_cure"),
                Hash40::new("top"),
                0,
                12,
                0,
                0,
                0,
                0,
                1,
                true,
            );
        }
    } else if WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND,
    ) == 2
    {
        if macros::is_excute(agent) {
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_mainbody"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_blitz"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_element"),
                true,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_guardian"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_second"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_strike"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_ultima"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_light"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_dark"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_double"),
                false,
            );

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
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(
                    agent,
                    Hash40::new("trail_stopga"),
                    Hash40::new("top"),
                    1.3,
                    11.5,
                    -0.3,
                    0,
                    0,
                    0,
                    1.03,
                    true,
                );
            }
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(
                agent,
                Hash40::new("trail_stopga_number"),
                Hash40::new("top"),
                1.3,
                11.5,
                -0.3,
                -5,
                -135,
                -3.5,
                1,
                true,
            );
        } else {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(
                    agent,
                    Hash40::new("trail_stopga"),
                    Hash40::new("top"),
                    -1.3,
                    11.5,
                    0.3,
                    0,
                    0,
                    0,
                    1.03,
                    true,
                );
            }
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(
                agent,
                Hash40::new("trail_stopga_number"),
                Hash40::new("top"),
                -1.3,
                11.5,
                0.3,
                -5,
                135,
                3.5,
                1,
                true,
            );
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_stopga_flash"),
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
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(
                agent,
                Hash40::new("sys_v_smoke_a"),
                Hash40::new("top"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.65,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
            );
        }
    } else if WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND,
    ) == 3
    {
        if macros::is_excute(agent) {
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_mainbody"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_blitz"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_element"),
                true,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_guardian"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_second"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_strike"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_ultima"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_light"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_dark"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("trail_double"),
                false,
            );

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

            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_air_hold"),
                Hash40::new("haver"),
                0,
                12,
                0,
                0,
                0,
                0,
                1,
                true,
            );
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_air"),
                Hash40::new("top"),
                0,
                12,
                0,
                0,
                0,
                0,
                1,
                true,
            );
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(
                agent,
                Hash40::new("sys_v_smoke_a"),
                Hash40::new("top"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.65,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
            );
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("trail_air_shot"),
                Hash40::new("haver"),
                0,
                12,
                0,
                0,
                0,
                0,
                1,
                true,
            );
        }
    }
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_appealhil", effect_appealhi, Low)
        .effect_acmd("effect_appealhir", effect_appealhi, Low)
        .install();
}
