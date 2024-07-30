use {
    crate::BLITZ_FLAG,
    crate::DEFAULT_FLAG,
    crate::ELEMENT_FLAG,
    crate::GUARDIAN_FLAG,
    crate::SECOND_FLAG,
    crate::STRIKE_FLAG,
    crate::ULTIMA_FLAG,
    crate::LIGHT_FLAG,
    crate::DARK_FLAG,
    crate::DOUBLE_FLAG,
    smash::{
        app::{lua_bind::*, sv_system, utility},
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
    },
    smashline::*,
};

unsafe extern "C" fn trail_ultima_check(agent: &mut L2CFighterCommon) {
    unsafe {
        let color_num =
            WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        if color_num == 3 {
            if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
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
                    false,
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
                    true,
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
                ULTIMA_FLAG = true;
                LIGHT_FLAG = false;
                DARK_FLAG = false;
                DOUBLE_FLAG = false;
            }
        }
    }
}
pub fn install() {
    Agent::new("trail")
        .on_line(Main, trail_ultima_check)
        .install();
}
