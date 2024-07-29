use {
    crate::RAGE_FLAG,
    smash::{
        app::{lua_bind::*, sv_system, utility},
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
    },
    smashline::*,
};

unsafe extern "C" fn trail_rage_check(agent: &mut L2CFighterCommon) {
    unsafe {
        let color_num =
            WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let damage = DamageModule::damage(agent.module_accessor, 0);

        if color_num == 3 {
            if damage >= 100.0 {
                VisibilityModule::set_whole(agent.module_accessor, false);
                ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("rage"), true);
                RAGE_FLAG = true;
            } else {
                VisibilityModule::set_whole(agent.module_accessor, true);
                ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("rage"), false);
                RAGE_FLAG = false;
            }
        }
    }
}
pub fn install() {
    Agent::new("trail")
        .on_line(Main, trail_rage_check)
        .install();
}
