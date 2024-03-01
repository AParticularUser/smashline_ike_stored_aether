use crate::common::consts::global_table;
use crate::imports::*;
use crate::ike::consts::{
    vars::*, 
    param
};


//removed special-fall from side-special
unsafe extern "C" fn fall_special_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::PREV_STATUS_KIND].get_i32() == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END 
    || agent.global_table[global_table::PREV_STATUS_KIND].get_i32() == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK {
        agent.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), true.into());
    }else {
        let original = smashline::original_status(Pre, agent, *FIGHTER_STATUS_KIND_FALL_SPECIAL);
        original(agent);
    }
    0.into()
}
unsafe extern "C" fn fall_aerial_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_FALL_AERIAL);
    let ret = original(agent);
    if agent.global_table[global_table::PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        let frame = MotionModule::end_frame_from_hash(agent.module_accessor, Hash40::new("special_air_s_end"));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_s_end"), frame, 1.0, false, 0.0, false, false);
        WorkModule::set_int64(agent.module_accessor, hash40("fall_special_f") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
        WorkModule::set_int64(agent.module_accessor, hash40("fall_special_b") as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("fall_special"), 0.0, 1.0, false, 0.0, false, false);
    }
    ret
}
//motion
unsafe extern "C" fn special_s_attack_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
            AttackModule::set_power_add_status(agent.module_accessor, param::IKE_FLOAT_STORED_AETHER_MUL);
            DamageModule::add_damage(agent.module_accessor, 2.0, 0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 60, 88, 0, 70, 6.5, 0.0, 8.4, 14.8, Some(0.0), Some(8.4), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
        }
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_ATTACK_END);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
unsafe extern "C" fn special_s_attack_eff(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 3);
        }
    }else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 2.0);
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
            frame(agent.lua_state_agent, 3.0);
            if macros::is_excute(agent) {
                macros::EFFECT_DETACH_KIND(agent, Hash40::new("ike_iaigiri_attack"), -1);
            }
        }else {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri_attack_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
                EffectModule::set_disable_render_offset_last(agent.module_accessor);
            }
            frame(agent.lua_state_agent, 3.0);
            if macros::is_excute(agent) {
                macros::EFFECT_DETACH_KIND(agent, Hash40::new("ike_iaigiri_attack_r"), -1);
            }
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ike_sword"), true, true);
        }
    }
}
unsafe extern "C" fn special_s_dash_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    loop {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
                macros::EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            }
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 4.0);
    }
}
unsafe extern "C" fn special_air_s_dash_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
        }else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ike_iaigiri"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
        frame(agent.lua_state_agent, 9.0);
        loop {
            if macros::is_excute(agent) {
                    macros::EFFECT_FOLLOW(agent, Hash40::new("ike_final_sword_fire"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, false);
            }
            wait(agent.lua_state_agent, 8.0);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_FALL_SPECIAL, fall_special_status_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_FALL_AERIAL, fall_aerial_status_main);

    agent.game_acmd("game_specialsattack", special_s_attack_game);
    agent.game_acmd("game_specialairsattack", special_s_attack_game);
    agent.effect_acmd("effect_specialsattack", special_s_attack_eff);
    agent.effect_acmd("effect_specialairsattack", special_s_attack_eff);
    agent.effect_acmd("effect_specialsdash", special_s_dash_eff);
    agent.effect_acmd("effect_specialairsdash", special_air_s_dash_eff);
}