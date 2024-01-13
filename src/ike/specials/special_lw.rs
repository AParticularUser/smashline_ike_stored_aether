use crate::imports::*;
use crate::ike::consts::{
    vars::*, 
    param
};

//game
unsafe extern "C" fn special_lw_hit_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_EATHER) {
            DamageModule::add_damage(agent.module_accessor, 0.3, 0);
            AttackModule::set_power_add_status(agent.module_accessor, param::IKE_FLOAT_STORED_EATHER_MUL);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 48, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        }else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 48, 9.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        }
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if macros::is_excute(agent) {
            AttackModule::set_optional_hit_sound(agent.module_accessor, 0, Hash40::new("se_ike_criticalhit"));
        }
    }
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 1.3);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
//effect
unsafe extern "C" fn special_lw_hit_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::FLASH(agent, 1, 1, 1, 0);
        if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_EATHER) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }else {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::COL_NORMAL(agent);
    }
}
unsafe extern "C" fn special_air_lw_hit_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 16, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        macros::EFFECT(agent, Hash40::new("ike_counter_success"), Hash40::new("top"), 0, 16, -1, 0, 90, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::FLASH(agent, 1, 1, 1, 0);
        if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_EATHER) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ike_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }else {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 5, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), 0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.1);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::COL_NORMAL(agent);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //game
    agent.game_acmd("game_speciallwhit", special_lw_hit_game);
    agent.game_acmd("game_specialairlwhit", special_lw_hit_game);
    //effect
    agent.effect_acmd("effect_speciallwhit", special_lw_hit_eff);
    agent.effect_acmd("effect_specialairlwhit", special_air_lw_hit_eff);
}