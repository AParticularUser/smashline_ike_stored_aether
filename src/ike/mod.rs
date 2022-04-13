use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::{lua_bind::*, sv_animcmd::*, *};
use smashline::*;
use smash_script::*;

static IKE_INT_STORED_EATHER_MUL : f32 = 1.2;

static mut IKE_FLAG_SPECIAL_N_STORED : [bool; 8] = [false; 8];
static mut IKE_FLAG_SPECIAL_N_STORED_MDL : [bool; 8] = [false; 8];
static mut IKE_FLOAT_SPECIAL_N_CHARGE_COUNT : [f32; 8] = [0.0; 8];
static mut IKE_FLAG_SPECIAL_N_EFFECT_SWORD_TEMP_DISABLE : [bool; 8] = [false; 8];

// logic for stored Aether
#[fighter_frame( agent = FIGHTER_KIND_IKE )]
fn ike_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        // if stored Eather force max charge
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
        && MotionModule::end_frame(fighter.module_accessor) - MotionModule::frame(fighter.module_accessor) <= 2.0 {
            if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
                IKE_FLAG_SPECIAL_N_STORED[entry_id] = false;
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_final_sword"), false, true);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX, false);
            // if not holding special force sub charge
            }else if IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id] > 0.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == false {
                    if IKE_FLAG_SPECIAL_N_STORED_MDL[entry_id] {
                        IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id] = 0.0;
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL, false);
                    }else {
                        IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id] = 0.0;
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, false);
                    }
                }
                // recall stored charge
                WorkModule::set_float(fighter.module_accessor, IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id], *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
            }
        // neutral-special cancel
        }else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP {
            // shield/air-dodge
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) == false {
                // if at max charge store Eather
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
                    IKE_FLAG_SPECIAL_N_STORED[entry_id] = true;
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 0.8, 0, 0, 0, 0, 0.6, false);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 6, 0, 0, 0, 0, 0.6, false);
                }else {
                    IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id] = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
                        IKE_FLAG_SPECIAL_N_STORED_MDL[entry_id] = true;
                    }
                }
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }else {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
            // jump
            }else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
                    // if at max charge store Eather
                    IKE_FLAG_SPECIAL_N_STORED[entry_id] = true;
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 0.8, 0, 0, 0, 0, 0.6, false);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 6, 0, 0, 0, 0, 0.6, false);
                }else {
                    IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id] = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);if WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
                        IKE_FLAG_SPECIAL_N_STORED_MDL[entry_id] = true;
                    }
                }
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }else {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            // platform drop
            }else if ControlModule::get_stick_y(fighter.module_accessor) <= -0.9 {
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    GroundModule::pass_floor(fighter.module_accessor);
                }
            }
        }else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX {
            IKE_FLAG_SPECIAL_N_STORED_MDL[entry_id] = false;
            IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id] = 0.0;
        }else if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
            // lose stored Eather
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FINAL
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_WIN
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_LOSE {
                IKE_FLAG_SPECIAL_N_STORED[entry_id] = false;
                IKE_FLAG_SPECIAL_N_STORED_MDL[entry_id] = false;
                IKE_FLOAT_SPECIAL_N_CHARGE_COUNT[entry_id] = 0.0;
                IKE_FLAG_SPECIAL_N_EFFECT_SWORD_TEMP_DISABLE[entry_id] = false;
            }else {
                if IKE_FLAG_SPECIAL_N_EFFECT_SWORD_TEMP_DISABLE[entry_id] {
                    if MotionModule::frame(fighter.module_accessor) >= 24.0
                    || StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2 {
                        IKE_FLAG_SPECIAL_N_EFFECT_SWORD_TEMP_DISABLE[entry_id] = false;
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 0.8, 0, 0, 0, 0, 0.6, false);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 6, 0, 0, 0, 0, 0.6, false);
                    }else if MotionModule::frame(fighter.module_accessor) <= 2.0 {
                        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_final_sword"), true, true);
                    }
                }
            }
        }
    }
}


// changes effects if stored Eather--------------------------------------------------------------------------------------------
// aerials
#[acmd_script( agent = "ike", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_forward_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_back_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attackairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_down_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("ike_attack_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("ike_attack_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attackairn", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_neutral_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
        */
    }
}
// misc
#[acmd_script( agent = "ike", script = "effect_attack13", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_jab_3_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 16, 0, 0.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 10, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 16, 0, 0.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 10, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attackdash", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_dash_attack_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_cliffattack", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_ledge_attack_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_downattackd", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_getup_attack_d_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_downattacku", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_getup_attack_u_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_slipattack", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_trip_attack_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
        */
    }
}
// tilts
#[acmd_script( agent = "ike", script = "effect_attacks3", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_side_tilt_side_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attacks3hi", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_side_tilt_up_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attacks3lw", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_side_tilt_down_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_tilt_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 5);
        }
        frame(fighter.lua_state_agent, 29.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 5);
        }
        frame(fighter.lua_state_agent, 29.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_down_tilt_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 7, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
        */
    }
}
// smashes
#[acmd_script( agent = "ike", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_side_smash_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 29.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 34.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 18, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 16, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 29.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword1"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 34.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 18, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 16, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attackhi4", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_smash_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 29.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -3, 0, -9, 0, 30, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2.5, 0, -7, 0, 30, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword1"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 29.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -3, 0, -9, 0, 30, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2.5, 0, -7, 0, 30, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_attacklw4", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_down_smash_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 31.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword1"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword1"), Hash40::new("tex_ike_sword2"), 6, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 31.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
        */
    }
}
// neutral special
#[acmd_script( agent = "ike", script = "effect_specialnloop", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_neutral_special_loop_effect(fighter: &mut L2CAgentBase) {
    while WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.125, 0.4, 1, 0.2);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.5, 15, 0, 4, 0, 0, 0, false);
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    while WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.125, 0.4, 1, 0.3);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.72, 15, 0, 4, 0, 0, 0, false);
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    while WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.125, 0.4, 1, 0.4);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
}
#[acmd_script( agent = "ike", script = "effect_specialairnloop", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_neutral_special_loop_air_effect(fighter: &mut L2CAgentBase) {
    while WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.125, 0.4, 1, 0.2);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    while WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.125, 0.4, 1, 0.3);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    while WorkModule::get_int(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0.125, 0.4, 1, 0.4);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
}
// down special
#[acmd_script( agent = "ike", script = "effect_speciallwhit", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_down_special_hit_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::FLASH(fighter, 1, 1, 1, 0);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("null"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.5, 0.1);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), false, true);
            macros::AFTER_IMAGE_OFF(fighter, 4);
            macros::COL_NORMAL(fighter);
        }
        */
    }else {
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::FLASH(fighter, 1, 1, 1, 0);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
            //macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), false, true);
            macros::AFTER_IMAGE_OFF(fighter, 4);
            macros::COL_NORMAL(fighter);
        }
    }
}
#[acmd_script( agent = "ike", script = "effect_specialairlwhit", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_down_special_hit_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::FLASH(fighter, 1, 1, 1, 0);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0, 1, 0, Hash40::new("sword"), 0, 16.2, 0, true, Hash40::new("null"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.5, 0.1);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), false, true);
            macros::AFTER_IMAGE_OFF(fighter, 4);
            macros::COL_NORMAL(fighter);
        }
        */
    }else {
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::FLASH(fighter, 1, 1, 1, 0);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
            //macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), false, true);
            macros::AFTER_IMAGE_OFF(fighter, 4);
            macros::COL_NORMAL(fighter);
        }
    }
}
// side special
#[acmd_script( agent = "ike", script = "effect_specialsattack", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_side_special_hit_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), true, true);
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 2.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_iaigiri_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
            }
            frame(fighter.lua_state_agent, 3.0);
            if macros::is_excute(fighter) {
                macros::EFFECT_DETACH_KIND(fighter, Hash40::new("ike_iaigiri_attack"), -1);
            }
        }else {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_iaigiri_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
            }
            frame(fighter.lua_state_agent, 3.0);
            if macros::is_excute(fighter) {
                macros::EFFECT_DETACH_KIND(fighter, Hash40::new("ike_iaigiri_attack"), -1);
            }
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_specialairsattack", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_side_special_hit_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), true, true);
            macros::AFTER_IMAGE_OFF(fighter, 3);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 2.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_iaigiri_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
            }
            frame(fighter.lua_state_agent, 3.0);
            if macros::is_excute(fighter) {
                macros::EFFECT_DETACH_KIND(fighter, Hash40::new("ike_iaigiri_attack"), -1);
            }
        }else {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_iaigiri_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
            }
            frame(fighter.lua_state_agent, 3.0);
            if macros::is_excute(fighter) {
                macros::EFFECT_DETACH_KIND(fighter, Hash40::new("ike_iaigiri_attack"), -1);
            }
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
        }
        */
    }
}
// up special
#[acmd_script( agent = "ike", script = "effect_specialhi1", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_special_1_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            IKE_FLAG_SPECIAL_N_EFFECT_SWORD_TEMP_DISABLE[entry_id] = true;
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_specialairhi1", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_special_1_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            IKE_FLAG_SPECIAL_N_EFFECT_SWORD_TEMP_DISABLE[entry_id] = true;
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 8, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_specialhi2", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_special_2_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_start"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0, 220.0, 220.0);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_tenku_start"), true, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_start"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_tenku_start"), true, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_specialairhi2", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_special_2_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_start"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0, 220.0, 220.0);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_tenku_start"), true, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_start"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_tenku_start"), true, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_specialhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_special_3_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 10, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword3"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "effect_specialairhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_special_3_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 10, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword3"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        */
    }
}
#[acmd_script( agent = "ike", scripts = ["effect_specialhi4"], category = ACMD_EFFECT, low_priority )]
unsafe fn ike_up_special_4_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::AFTER_IMAGE_OFF(fighter, 3);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"),Hash40::new("top"), 12, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("ike_volcano_max"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_SCALE_W(fighter, 1.2, 1, 1.2);
            macros::EFFECT(fighter, Hash40::new("ike_volcano_add4"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("ike_volcano_flash3"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("ike_volcano_add2"), Hash40::new("top"), 0, 0, 12, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword2"), true, true);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_tenku_sword3"), false, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("ike_tenku_ground"), Hash40::new("top"), 10, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
        }
        */
    }
}
// sword up special
#[acmd_script( agent = "ike_sword", script = "effect_specialhi2", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_sword_up_special_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        frame(fighter.lua_state_agent, 1.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_tenku"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }else {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_tenku"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_final_sword_tenku"), true, true);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 1.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }else {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_tenku_sword"), true, true);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword2"), Hash40::new("haver"), 0, 0, 0, 0, 180, 0, 1, true);
        }
        */
    }
}
#[acmd_script( agent = "ike_sword", script = "effect_specialairhi2", category = ACMD_EFFECT, low_priority )]
unsafe fn ike_sword_up_special_air_effect(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword2"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        frame(fighter.lua_state_agent, 1.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_tenku"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }else {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_tenku"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_final_sword_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_final_sword_tenku"), true, true);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 1.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }else {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword"), Hash40::new("haver"), 0, 6.2, 0, 0, 0, 0, 1, true);
            }
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_tenku_sword"), true, true);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_tenku_sword2"), Hash40::new("haver"), 0, 0, 0, 0, 180, 0, 1, true);
        }
        */
    }
}
// -----------------------------------------------------------------------------------------------------------------------------------------

// changes damage if stored Eather--------------------------------------------------------------------------------------------
// specials
#[acmd_script( agent = "ike", script = "game_specialhi2", category = ACMD_GAME, low_priority )]
unsafe fn ike_up_special_rise(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            //DamageModule::add_damage(fighter.module_accessor, 0.5, 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, -1);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0*IKE_INT_STORED_EATHER_MUL, 367, 100, 50, 0, 9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 41.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 44.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(0));
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, -1);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
            WorkModule::set_float(fighter.module_accessor, 4, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
            WorkModule::set_float(fighter.module_accessor, 4, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 100, 50, 0, 9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 41.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 44.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, 0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(0));
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_specialairhi2", category = ACMD_GAME, low_priority )]
unsafe fn ike_up_special_rise_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            //DamageModule::add_damage(fighter.module_accessor, 0.5, 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, -1);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
            WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0*IKE_INT_STORED_EATHER_MUL, 367, 100, 50, 0, 9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 41.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 44.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(0));
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_SWORD_FINAL);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, false, -1);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 88, 100, 150, 0, 3.5, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 94, 100, 150, 0, 1.5, 0.0, 3.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 96, 100, 150, 0, 3.5, 0.0, 5.0, 19.0, Some(0.0), Some(5.0), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 86, 100, 145, 0, 3.5, 0.0, 13.5, 7.0, Some(0.0), Some(13.5), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 6.0, 94, 100, 145, 0, 1.5, 0.0, 11.5, 15.0, Some(0.0), Some(15.5), Some(15.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 6.0, 96, 100, 140, 0, 3.5, 0.0, 13.5, 19.0, Some(0.0), Some(13.5), Some(7.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
            WorkModule::set_float(fighter.module_accessor, 4, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
            WorkModule::set_float(fighter.module_accessor, 4, *FIGHTER_IKE_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 100, 50, 0, 9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 41.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 44.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, 0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_IKE_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(0));
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_specialhi3", category = ACMD_GAME, low_priority )]
unsafe fn ike_up_special_fall(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
            KineticModule::clear_speed_all(fighter.module_accessor);
            //macros::ADD_SPEED_NO_LIMIT(fighter, 0, -6);
            fighter.clear_lua_stack();
            lua_args!(fighter, 0, -6);
            sv_animcmd::ADD_SPEED_NO_LIMIT(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            ///////////////////////////////////////////////////////
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 3.0*IKE_INT_STORED_EATHER_MUL, 70, 30, 0, 120, 6.5, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-12.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 3.0*IKE_INT_STORED_EATHER_MUL, 270, 100, 165, 0, 4.8, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-13.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 3.0*IKE_INT_STORED_EATHER_MUL, 70, 50, 0, 0, 4.8, 0.0, 6.5, -1.0, Some(0.0), Some(12.5), Some(-1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 3.0*IKE_INT_STORED_EATHER_MUL, 270, 100, 150, 0, 4.8, 0.0, 6.5, -1.0, Some(0.0), Some(12.5), Some(-1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
            KineticModule::clear_speed_all(fighter.module_accessor);
            macros::ADD_SPEED_NO_LIMIT(fighter, 0, -6);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 30, 0, 120, 6.5, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-12.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 270, 100, 165, 0, 4.8, 0.0, 6.8, -1.0, Some(0.0), Some(6.8), Some(-13.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 50, 0, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 270, 100, 150, 0, 4.8, 0.0, 6.8, -1.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_specialhi4", category = ACMD_GAME, low_priority )]
unsafe fn ike_up_special_end(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 2.5, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 50, 155, 0, 70, 10.0, 0.0, 6.0, 11.8, Some(0.0), Some(11.0), Some(11.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 80, 100, 0, 40, 15.0, 0.0, 8.0, 11.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 80, 100, 0, 40, 10.0, 0.0, 25.0, 11.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 50, 155, 0, 70, 10.0, 0.0, 6.0, 11.8, Some(0.0), Some(11.0), Some(11.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_specialsattack", category = ACMD_GAME, low_priority )]
unsafe fn ike_side_special_hit(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 2.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_specialairsattack", category = ACMD_GAME, low_priority )]
unsafe fn ike_side_special_hit_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 2.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0*IKE_INT_STORED_EATHER_MUL, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_speciallwhit", category = ACMD_GAME, low_priority )]
unsafe fn ike_down_special_hit(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.3, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 48, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_ike_criticalhit"));
            }
        }
        frame(fighter.lua_state_agent, 7.0);
            macros::FT_MOTION_RATE(fighter, 1.3);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 48, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                AttackModule::set_optional_hit_sound(fighter.module_accessor, Hash40::new("se_ike_criticalhit"));
            }
        }
        frame(fighter.lua_state_agent, 7.0);
            macros::FT_MOTION_RATE(fighter, 1.3);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_specialairlwhit", category = ACMD_GAME, low_priority )]
unsafe fn ike_down_special_hit_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.3, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 48, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_ike_criticalhit"));
            }
        }
        frame(fighter.lua_state_agent, 7.0);
            macros::FT_MOTION_RATE(fighter, 1.3);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 48, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(fighter) {
                AttackModule::set_optional_hit_sound(fighter.module_accessor, Hash40::new("se_ike_criticalhit"));
            }
        }
        frame(fighter.lua_state_agent, 7.0);
            macros::FT_MOTION_RATE(fighter, 1.3);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
// smashes
#[acmd_script( agent = "ike", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn ike_side_smash(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(fighter.lua_state_agent, 31.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.5, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 19.0*IKE_INT_STORED_EATHER_MUL, 361, 90, 0, 40, 4.0, 0.0, 6.5, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 19.0*IKE_INT_STORED_EATHER_MUL, 361, 90, 0, 40, 3.0, 0.0, 11.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 33.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 22.0*IKE_INT_STORED_EATHER_MUL, 361, 85, 0, 49, 5.0, 0.0, 2.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 22.0*IKE_INT_STORED_EATHER_MUL, 361, 85, 0, 49, 5.7, 0.0, 7.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 22.0*IKE_INT_STORED_EATHER_MUL, 361, 85, 0, 49, 5.7, 0.0, 7.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 19.0*IKE_INT_STORED_EATHER_MUL, 57, 86, 0, 40, 5.5, 0.0, 11.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(fighter.lua_state_agent, 31.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 19.0, 361, 90, 0, 40, 4.0, 0.0, 6.5, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 19.0, 361, 90, 0, 40, 3.0, 0.0, 11.0, -2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 33.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 22.0, 361, 85, 0, 49, 5.0, 0.0, 2.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 22.0, 361, 85, 0, 49, 5.7, 0.0, 7.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 22.0, 361, 85, 0, 49, 5.7, 0.0, 7.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 19.0, 57, 86, 0, 40, 5.5, 0.0, 11.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, *ATTACK_HEIGHT_HIGH, false);
        }
        frame(fighter.lua_state_agent, 36.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn ike_up_smash(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.5, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 17.0*IKE_INT_STORED_EATHER_MUL, 82, 86, 0, 50, 5.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 17.0*IKE_INT_STORED_EATHER_MUL, 75, 86, 0, 50, 5.0, 0.0, 4.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 17.0*IKE_INT_STORED_EATHER_MUL, 70, 86, 0, 50, 5.0, 0.0, 7.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 17.0*IKE_INT_STORED_EATHER_MUL, 65, 86, 0, 50, 5.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0*IKE_INT_STORED_EATHER_MUL, 65, 85, 0, 50, 6.0, 0.0, 6.0, -16.0, Some(0.0), Some(6.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::clear(fighter.module_accessor, 1, false);
            AttackModule::clear(fighter.module_accessor, 2, false);
            AttackModule::clear(fighter.module_accessor, 3, false);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 17.0, 82, 86, 0, 50, 5.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 17.0, 75, 86, 0, 50, 5.0, 0.0, 4.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 17.0, 70, 86, 0, 50, 5.0, 0.0, 7.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 17.0, 65, 86, 0, 50, 5.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 65, 85, 0, 50, 6.0, 0.0, 6.0, -16.0, Some(0.0), Some(6.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::clear(fighter.module_accessor, 1, false);
            AttackModule::clear(fighter.module_accessor, 2, false);
            AttackModule::clear(fighter.module_accessor, 3, false);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn ike_down_smash(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.7, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 16.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 16.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 4.4, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 16.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 8.2, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 16.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 12.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.7, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 19.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 19.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 4.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 19.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 8.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 19.0*IKE_INT_STORED_EATHER_MUL, 48, 100, 0, 40, 2.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 9.0*IKE_INT_STORED_EATHER_MUL, 61, 100, 0, 3, 2.5, 0.0, 0.6, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 9.0*IKE_INT_STORED_EATHER_MUL, 61, 100, 0, 30, 2.5, 0.0, 4.4, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 9.0*IKE_INT_STORED_EATHER_MUL, 61, 100, 0, 30, 2.5, 0.0, 8.2, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 9.0*IKE_INT_STORED_EATHER_MUL, 61, 100, 0, 30, 2.5, 0.0, 12.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        frame(fighter.lua_state_agent, 37.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 16.0, 48, 100, 0, 40, 2.5, 0.0, 0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 16.0, 48, 100, 0, 40, 2.5, 0.0, 4.4, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 16.0, 48, 100, 0, 40, 2.5, 0.0, 8.2, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 16.0, 48, 100, 0, 40, 2.5, 0.0, 12.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, *ATTACK_HEIGHT_LOW, false);
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 19.0, 48, 100, 0, 40, 2.5, 0.0, 0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 19.0, 48, 100, 0, 40, 2.5, 0.0, 4.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 19.0, 48, 100, 0, 40, 2.5, 0.0, 8.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 19.0, 48, 100, 0, 40, 2.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, *ATTACK_HEIGHT_LOW, false);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 9.0, 61, 100, 0, 3, 2.5, 0.0, 0.6, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 9.0, 61, 100, 0, 30, 2.5, 0.0, 4.4, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 9.0, 61, 100, 0, 30, 2.5, 0.0, 8.2, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 9.0, 61, 100, 0, 30, 2.5, 0.0, 12.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, *ATTACK_HEIGHT_LOW, false);
        }
        frame(fighter.lua_state_agent, 37.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
// tilts
#[acmd_script( agent = "ike", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn ike_side_tilt_side(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        macros::FT_MOTION_RATE(fighter, 0.77);
        frame(fighter.lua_state_agent, 13.0);
        macros::FT_MOTION_RATE(fighter, 0.8);
        frame(fighter.lua_state_agent, 14.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5*IKE_INT_STORED_EATHER_MUL, 361, 97, 0, 30, 4.0, 0.0, 10.0, 10.0, Some(0.0), Some(8.0), Some(24.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        macros::FT_MOTION_RATE(fighter, 0.77);
        frame(fighter.lua_state_agent, 13.0);
        macros::FT_MOTION_RATE(fighter, 0.8);
        frame(fighter.lua_state_agent, 14.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 361, 97, 0, 30, 4.0, 0.0, 10.0, 10.0, Some(0.0), Some(8.0), Some(24.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attacks3hi", category = ACMD_GAME, low_priority )]
unsafe fn ike_side_tilt_up(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        macros::FT_MOTION_RATE(fighter, 0.77);
        frame(fighter.lua_state_agent, 13.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5*IKE_INT_STORED_EATHER_MUL, 361, 97, 0, 30, 4.0, 0.0, 11.0, 10.0, Some(0.0), Some(14.0), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        macros::FT_MOTION_RATE(fighter, 0.77);
        frame(fighter.lua_state_agent, 13.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 361, 97, 0, 30, 4.0, 0.0, 11.0, 10.0, Some(0.0), Some(14.0), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attacks3lw", category = ACMD_GAME, low_priority )]
unsafe fn ike_side_tilt_down(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        macros::FT_MOTION_RATE(fighter, 0.77);
        frame(fighter.lua_state_agent, 13.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5*IKE_INT_STORED_EATHER_MUL, 361, 97, 0, 30, 4.0, 0.0, 8.5, 10.0, Some(0.0), Some(3.5), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        macros::FT_MOTION_RATE(fighter, 0.77);
        frame(fighter.lua_state_agent, 13.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 361, 97, 0, 30, 4.0, 0.0, 8.5, 10.0, Some(0.0), Some(3.5), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn ike_up_tilt(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 12.0*IKE_INT_STORED_EATHER_MUL, 95, 90, 0, 50, 5.0, 0.5, 13.5, -1.0, Some(0.5), Some(2.8), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0*IKE_INT_STORED_EATHER_MUL, 85, 90, 0, 50, 5.0, 0.0, 12.5, -2.0, Some(0.0), Some(4.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 12.0, 95, 90, 0, 50, 5.0, 0.5, 13.5, -1.0, Some(0.5), Some(2.8), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0, 85, 90, 0, 50, 5.0, 0.0, 12.5, -2.0, Some(0.0), Some(4.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 22.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn ike_down_tilt(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0*IKE_INT_STORED_EATHER_MUL, 80, 55, 0, 70, 3.4, -1.0, 1.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 8.0*IKE_INT_STORED_EATHER_MUL, 80, 55, 0, 70, 3.4, -0.3, 4.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 8.0*IKE_INT_STORED_EATHER_MUL, 80, 55, 0, 70, 3.4, 0.7, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 8.0*IKE_INT_STORED_EATHER_MUL, 80, 55, 0, 70, 3.0, 1.5, 10.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.4, -1.0, 1.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.4, -0.3, 4.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.4, 0.7, 7., 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.0, 1.5, 10.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
// aerials
#[acmd_script( agent = "ike", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn ike_back_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.1, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 5.0, 0.0, 9.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 5.0, 0.0, 6.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 4.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 2, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 3.5, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 3.0, 0.0, 4.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0*IKE_INT_STORED_EATHER_MUL, 361, 100, 0, 30, 5.0, 0.0, 14.8, -11.0, Some(0.0), Some(12.0), Some(-17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 5.0, 0.0, 9.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 5.0, 0.0, 6.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 4.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 2, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 3.5, 0.0, 10.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 3.0, 0.0, 4.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("sword"), 14.0, 361, 100, 0, 30, 5.0, 0.0, 14.8, -11.0, Some(0.0), Some(12.0), Some(-17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn ike_forward_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 13.0*IKE_INT_STORED_EATHER_MUL, 30, 60, 0, 55, 5.0, 0.0, 9.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 13.0*IKE_INT_STORED_EATHER_MUL, 30, 60, 0, 55, 5.0, 0.0, 6.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 13.0*IKE_INT_STORED_EATHER_MUL, 30, 60, 0, 55, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 42.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }else {
        original!(fighter);
        /*
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 13.0, 30, 60, 0, 55, 5.0, 0.0, 9.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 13.0, 30, 60, 0, 55, 5.0, 0.0, 6.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 13.0, 30, 60, 0, 55, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 42.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn ike_up_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 11.0*IKE_INT_STORED_EATHER_MUL, 80, 94, 0, 55, 5.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 11.0*IKE_INT_STORED_EATHER_MUL, 80, 94, 0, 55, 5.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 11.0*IKE_INT_STORED_EATHER_MUL, 80, 94, 0, 55, 5.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 11.0*IKE_INT_STORED_EATHER_MUL, 80, 94, 0, 55, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 51.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 51.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn ike_down_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.9, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0*IKE_INT_STORED_EATHER_MUL, 50, 100, 0, 30, 6.0, 0.0, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0*IKE_INT_STORED_EATHER_MUL, 50, 100, 0, 55, 5.5, 0.0, -2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0*IKE_INT_STORED_EATHER_MUL, 50, 100, 0, 30, 6.3, 0.0, -6.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 48.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 50, 100, 0, 30, 6.0, 0.0, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 50, 100, 0, 55, 5.5, 0.0, -2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 50, 100, 0, 30, 6.3, 0.0, -6.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 48.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn ike_neutral_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 7.5*IKE_INT_STORED_EATHER_MUL, 80, 130, 0, 35, 4.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 7.5*IKE_INT_STORED_EATHER_MUL, 80, 130, 0, 35, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 7.5*IKE_INT_STORED_EATHER_MUL, 80, 130, 0, 35, 4.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.0*IKE_INT_STORED_EATHER_MUL, 80, 130, 0, 35, 3.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 6.0*IKE_INT_STORED_EATHER_MUL, 80, 130, 0, 35, 3.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 6.0*IKE_INT_STORED_EATHER_MUL, 80, 130, 0, 35, 3.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 50.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 7.5, 80, 130, 0, 35, 4.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 7.5, 80, 130, 0, 35, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 7.5, 80, 130, 0, 35, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.0, 80, 130, 0, 35, 3.0, 0.0, 10.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 6.0, 80, 130, 0, 35, 3.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 6.0, 80, 130, 0, 35, 3.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 50.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        */
    }
}
// misc
#[acmd_script( agent = "ike", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn ike_dash_attack(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.8);
        frame(fighter.lua_state_agent, 18.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 1.1, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0*IKE_INT_STORED_EATHER_MUL, 52, 88, 0, 70, 7.0, 0.0, 10.0, 14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0*IKE_INT_STORED_EATHER_MUL, 52, 88, 0, 70, 3.0, 0.0, 15.0, 19.0, Some(0.0), Some(15.0), Some(20.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0*IKE_INT_STORED_EATHER_MUL, 52, 88, 0, 70, 5.0, 0.0, 8.0, 11.5, Some(0.0), Some(8.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0,  1, 1.2);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0*IKE_INT_STORED_EATHER_MUL, 45, 110, 0, 60, 2.0, 0.0, 17.0, 20.5, Some(0.0), Some(15.0), Some(9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::clear(fighter.module_accessor, 1, false);
            AttackModule::clear(fighter.module_accessor, 2, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.8);
        frame(fighter.lua_state_agent, 18.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 52, 88, 0, 70, 7.0, 0.0, 10.0, 14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 52, 88, 0, 70, 3.0, 0.0, 15.0, 19.0, Some(0.0), Some(15.0), Some(20.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 52, 88, 0, 70, 5.0, 0.0, 8.0, 11.5, Some(0.0), Some(8.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0,  1, 1.2);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 110, 0, 60, 2.0, 0.0, 17.0, 20.5, Some(0.0), Some(15.0), Some(9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::clear(fighter.module_accessor, 1, false);
            AttackModule::clear(fighter.module_accessor, 2, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_attack13", category = ACMD_GAME, low_priority )]
unsafe fn ike_jab_3(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.8, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0*IKE_INT_STORED_EATHER_MUL, 45, 100, 0, 60, 4.0, 0.0, 12.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 5.0*IKE_INT_STORED_EATHER_MUL, 45, 100, 0, 60, 4.0, 0.0, 9.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 5.0*IKE_INT_STORED_EATHER_MUL, 45, 100, 0, 60, 4.0, 0.0, 5.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 5.0*IKE_INT_STORED_EATHER_MUL, 45, 100, 0, 60, 3.3, 0.0, 1.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 100, 0, 60, 4.0, 0.0, 12.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 45, 100, 0, 60, 4.0, 0.0, 9.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 45, 100, 0, 60, 4.0, 0.0, 5.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 45, 100, 0, 60, 3.3, 0.0, 1.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, *ATTACK_HEIGHT_HIGH, false);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_cliffattack", category = ACMD_GAME, low_priority )]
unsafe fn ike_ledge_attack(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.5, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0*IKE_INT_STORED_EATHER_MUL, 45, 20, 0, 90, 5.0, 0.0, 5.0, 12.0, Some(0.0), Some(5.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, 12.0, Some(0.0), Some(5.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_downattackd", category = ACMD_GAME, low_priority )]
unsafe fn ike_getup_attack_down(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.5, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0*IKE_INT_STORED_EATHER_MUL, 48, 48, 0, 80, 5.5, 0.0, 5.0, -18.5, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0*IKE_INT_STORED_EATHER_MUL, 48, 48, 0, 80, 5.5, 0.0, 5.0, 15.5, Some(0.0), Some(5.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.5, 0.0, 5.0, -18.5, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.5, 0.0, 5.0, 15.5, Some(0.0), Some(5.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_downattacku", category = ACMD_GAME, low_priority )]
unsafe fn ike_getup_attack_up(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.5, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0*IKE_INT_STORED_EATHER_MUL, 48, 48, 0, 80, 5.5, 0.0, 5.0, -19.5, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0*IKE_INT_STORED_EATHER_MUL, 48, 48, 0, 80, 5.5, 0.0, 5.0, 17.5, Some(0.0), Some(5.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.5, 0.0, 5.0, -19.5, Some(0.0), Some(5.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.5, 0.0, 5.0, 17.5, Some(0.0), Some(5.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
#[acmd_script( agent = "ike", script = "game_slipattack", category = ACMD_GAME, low_priority )]
unsafe fn ike_trip_attack(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            DamageModule::add_damage(fighter.module_accessor, 0.5, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0*IKE_INT_STORED_EATHER_MUL, 361, 50, 0, 60, 5.0, 0.0, 5.0, -20.5, Some(0.0), Some(5.0), Some(-11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0*IKE_INT_STORED_EATHER_MUL, 361, 50, 0, 60, 5.0, 0.0, 5.0, 19.5, Some(0.0), Some(5.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }else {
        original!(fighter);
        /*
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, -20.5, Some(0.0), Some(5.0), Some(-11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, 19.5, Some(0.0), Some(5.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        */
    }
}
// ----------------------------------------------------------------------------------------------------------------------------------------
// added boom
#[acmd_script( agent = "ike", script = "sound_specialhi4", category = ACMD_SOUND, low_priority )]
unsafe fn ike_up_special_end_sound(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IKE_FLAG_SPECIAL_N_STORED[entry_id] {
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_ike_special_h05"));
            macros::PLAY_SE(fighter, Hash40::new("se_ike_special_h06"));
            macros::PLAY_SE(fighter, Hash40::new("se_ike_swordgroundhit"));
            macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n08"));
        }
    }else {
        original!(fighter);
        /*
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_ike_special_h05"));
            macros::PLAY_SE(fighter, Hash40::new("se_ike_special_h06"));
            macros::PLAY_SE(fighter, Hash40::new("se_ike_swordgroundhit"));
        }
        */
    }
}
// removed free-fall
#[acmd_script( agent = "ike", script = "game_specialairsend", category = ACMD_GAME, low_priority )]
unsafe fn ike_side_special_end(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_END_NO_LANDING);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        ike_frame
    );
    smashline::install_acmd_scripts!(
        // effect ---------------------
        // aerials
        ike_forward_air_effect,
        ike_back_air_effect,
        ike_down_air_effect,
        ike_up_air_effect,
        ike_neutral_air_effect,
        // misc
        ike_jab_3_effect,
        ike_dash_attack_effect,
        ike_ledge_attack_effect,
        ike_getup_attack_d_effect,
        ike_getup_attack_u_effect,
        ike_trip_attack_effect,
        // tilts
        ike_side_tilt_side_effect,
        ike_side_tilt_up_effect,
        ike_side_tilt_down_effect,
        ike_up_tilt_effect,
        ike_down_tilt_effect,
        // smashes
        ike_up_smash_effect,
        ike_side_smash_effect,
        ike_down_smash_effect,
        // specials
        ike_neutral_special_loop_effect,
        ike_neutral_special_loop_air_effect,
        ike_up_special_1_effect,
        ike_up_special_1_air_effect,
        ike_up_special_2_effect,
        ike_up_special_2_air_effect,
        ike_up_special_3_effect,
        ike_up_special_3_air_effect,
        ike_up_special_4_effect,
        ike_side_special_hit_effect,
        ike_side_special_hit_air_effect,
        ike_down_special_hit_effect,
        ike_down_special_hit_air_effect,
        // articles/weapons
        ike_sword_up_special_effect,
        ike_sword_up_special_air_effect,
        // game ----------------------
        // specials
        ike_up_special_rise,
        ike_up_special_rise_air,
        ike_up_special_fall,
        ike_up_special_end,
        ike_side_special_hit,
        ike_side_special_hit_air,
        ike_down_special_hit,
        ike_down_special_hit_air,
        // smashes
        ike_side_smash,
        ike_up_smash,
        ike_down_smash,
        // tilts
        ike_side_tilt_side,
        ike_side_tilt_up,
        ike_side_tilt_down,
        ike_up_tilt,
        ike_down_tilt,
        // aerials
        ike_back_air,
        ike_forward_air,
        ike_up_air,
        ike_down_air,
        ike_neutral_air,
        // misc
        ike_dash_attack,
        ike_jab_3,
        ike_ledge_attack,
        ike_getup_attack_down,
        ike_getup_attack_up,
        ike_trip_attack,
        // other ---------------
        ike_up_special_end_sound,
        ike_side_special_end
    );
}