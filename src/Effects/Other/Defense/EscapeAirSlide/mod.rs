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

unsafe extern "C" fn effect_escapeairslide(agent: &mut L2CAgentBase) {
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
}

pub fn install() {
    Agent::new("trail")
        .effect_acmd("effect_escapeairslide", effect_escapeairslide, Low)
        .install();
}
