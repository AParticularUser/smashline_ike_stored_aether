use crate::imports::*;
use crate::common::consts::*;
use crate::ike::consts::{
    vars::*, 
    param
};

//stored max charge effects
pub unsafe fn stored_aether_effect_on(agent: &mut L2CAgentBase) {
    VarModule::set_int(agent.module_accessor, instance::IKE_INT_STORED_AETHER_EFFECT_COUNT, param::IKE_INT_STORED_AETHER_EFFECT_FRAME);
    macros::EFFECT_FOLLOW(agent, Hash40::new("ike_sword2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
    macros::EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 0.8, 0, 0, 0, 0, 0.6, false);
    macros::EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword"), Hash40::new("sword"), 0, 6, 0, 0, 0, 0, 0.6, false);
}
pub unsafe fn stored_aether_effect_off(agent: &mut L2CAgentBase) {
    macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_sword2"), true, true);
    macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_final_sword"), true, true);
}
////status
//start
unsafe extern "C" fn special_n_start_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP
    && agent.global_table[global_table::STATUS_KIND].get_i32() != *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END {
        if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
            VarModule::off_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER);
            stored_aether_effect_off(agent);
        }
        VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, 0.0);
    }
    let original = smashline::original_status(End, agent, *FIGHTER_STATUS_KIND_SPECIAL_N);
    original(agent)
}
//loop
unsafe extern "C" fn special_n_loop_status_init(agent: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Init, agent, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP);
    original(agent);
    if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
        let param_charge_max = WorkModule::get_param_int(agent.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_max"));        
        let charge_level = ((param_charge_max-1)*30) as f32;
        WorkModule::set_float(agent.module_accessor, charge_level, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
        WorkModule::set_int(agent.module_accessor, 2,*FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_MAX_CHARGE_SOUND_DONE);
        stored_aether_effect_off(agent);
    }else {
        let param_charge_mdl = WorkModule::get_param_int(agent.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
        let charge_count_mdl = (param_charge_mdl*30) as f32;
        if VarModule::get_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT) > charge_count_mdl {
            WorkModule::set_int(agent.module_accessor, 1,*FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
        }
        let charge_count = VarModule::get_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT);
        WorkModule::set_float(agent.module_accessor, charge_count, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
    }
    VarModule::off_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER);
    VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, 0.0);

    0.into()
}
unsafe extern "C" fn special_n_loop_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let param_charge_mdl = WorkModule::get_param_int(agent.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
    let charge_count_mdl = (param_charge_mdl*30) as f32;
    let param_charge_max = WorkModule::get_param_int(agent.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_max"));        
    let charge_count_max = ((param_charge_max-1)*30) as f32;
    if WorkModule::get_float(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) > charge_count_mdl 
    && WorkModule::get_int(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) < 1 {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 14, 0, 0, 0, 0, 1.7, false);
    }
    let jump_count_max = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < jump_count_max {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
            VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, charge_count_max);
            VarModule::on_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER);
            stored_aether_effect_on(agent);
        }else {
            let charge_count = WorkModule::get_float(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
            VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, charge_count);
        }
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
        }
    }else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) == false {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 2 {
            VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, charge_count_max);
            VarModule::on_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER);
            stored_aether_effect_on(agent);
        }else {
            let charge_count = WorkModule::get_float(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
            VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, charge_count);
        }
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if ControlModule::get_stick_y(agent.module_accessor) < -0.6 {
                agent.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            }else if ControlModule::get_stick_x(agent.module_accessor)*PostureModule::lr(agent.module_accessor) > 0.6 {
                agent.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            }else if ControlModule::get_stick_x(agent.module_accessor)*PostureModule::lr(agent.module_accessor) < -0.6 {
                agent.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            }else {
                agent.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
            }
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        }
    }else if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if ControlModule::get_flick_y(agent.module_accessor) <= 5 
        && ControlModule::get_stick_y(agent.module_accessor) < 0.0 
        && GroundModule::is_passable_ground(agent.module_accessor) {
            GroundModule::pass_floor(agent.module_accessor);
        }
    }
    let original = smashline::original_status(Exec, agent, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP);
    original(agent);
    //overwright wonky READY_LEVEL calculations to match main status
    if WorkModule::get_float(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) <= charge_count_mdl {
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
    }else if WorkModule::get_float(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) <= charge_count_max{
        WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
    }else {
        WorkModule::set_int(agent.module_accessor, 2, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL);
    }
    0.into()
}
//end
unsafe extern "C" fn special_n_end_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    let param_charge_mdl = WorkModule::get_param_int(agent.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
    let charge_count_mdl = (param_charge_mdl*30) as f32;
    if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
        VarModule::off_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER);
        stored_aether_effect_off(agent);
        agent.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX.into(), true.into());
    }else if VarModule::get_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT) > charge_count_mdl {
        agent.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL.into(), true.into());
    }else {
        let original = smashline::original_status(Pre, agent, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END);
        original(agent);
    }
    if VarModule::get_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT) > 0.0 {
        VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, 0.0);
    }
    0.into()
}
////motion
//loop
unsafe extern "C" fn special_n_loop_eff(agent: &mut L2CAgentBase) {
    let param_charge_mdl = WorkModule::get_param_int(agent.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_mdl"));        
    let charge_level_mdl = (param_charge_mdl*30) as f32;
    let param_charge_max = WorkModule::get_param_int(agent.module_accessor, smash::hash40("param_special_n"), smash::hash40("special_n_charge_count_max"));        
    let charge_level_max = ((param_charge_max-1)*30) as f32;
    // wait_loop_sync_mot();
    loop {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 0 {
            if WorkModule::get_float(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < charge_level_mdl/2.0 {
                if macros::is_excute(agent) {
                    macros::FLASH(agent, 0.125, 0.4, 1, 0.1);
                }
                wait(agent.lua_state_agent, 4.0);
                    if macros::is_excute(agent) {
                        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.5, 15, 0, 4, 0, 0, 0, false);
                        macros::COL_NORMAL(agent);
                    }
                wait(agent.lua_state_agent, 2.0);
            }else {
                if macros::is_excute(agent) {
                    macros::FLASH(agent, 0.125, 0.4, 1, 0.2);
                }
                wait(agent.lua_state_agent, 4.0);
                if macros::is_excute(agent) {
                    macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.62, 15, 0, 4, 0, 0, 0, false);
                    macros::COL_NORMAL(agent);
                }
                wait(agent.lua_state_agent, 2.0);
            }
        }else if WorkModule::get_int(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_INT_READY_LEVEL) == 1 {
            if WorkModule::get_float(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT) < ((charge_level_max-charge_level_mdl)/2.0)+charge_level_mdl {
                if macros::is_excute(agent) {
                    macros::FLASH(agent, 0.125, 0.4, 1, 0.3);
                }
                wait(agent.lua_state_agent, 4.0);
                if macros::is_excute(agent) {
                    macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.72, 15, 0, 4, 0, 0, 0, false);
                    macros::COL_NORMAL(agent);
                }
                wait(agent.lua_state_agent, 2.0);
            }else {
                if macros::is_excute(agent) {
                    macros::FLASH(agent, 0.125, 0.4, 1, 0.35);
                }
                wait(agent.lua_state_agent, 4.0);
                if macros::is_excute(agent) {
                    macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.87, 15, 0, 4, 0, 0, 0, false);
                    macros::COL_NORMAL(agent);
                }
                wait(agent.lua_state_agent, 2.0);
            }
        }else {
            if macros::is_excute(agent) {
                macros::FLASH(agent, 0.125, 0.4, 1, 0.4);
            }
            wait(agent.lua_state_agent, 4.0);
            if macros::is_excute(agent) {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
                macros::COL_NORMAL(agent);
            }
            wait(agent.lua_state_agent, 1.0);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //start
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_start_status_end);
    //loop
    agent.status(Init, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, special_n_loop_status_init);
    agent.status(Exec, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, special_n_loop_status_exec);
    agent.effect_acmd("effect_specialnloop", special_n_loop_eff);
    //end
    agent.status(Pre, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, special_n_end_status_pre);
}