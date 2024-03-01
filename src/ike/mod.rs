use crate::imports::*;
use crate::ike::{
    specials::special_n::*,
    consts::{
        vars::*, 
        param
    }
};


unsafe extern "C" fn ike_frame(agent: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(agent.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
                VarModule::off_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER);
                stored_aether_effect_off(agent);
            }else if VarModule::get_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT) > 0.0 {
                VarModule::set_float(agent.module_accessor, instance::IKE_FLOAT_SPECIAL_N_CHARGE_COUNT, 0.0);
            }
        }else if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER) {
            if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER_EFFECT_DISABLE) == false {
                if [
                    //normals
                    *FIGHTER_STATUS_KIND_ATTACK,
                    *FIGHTER_STATUS_KIND_ATTACK_100,
                    *FIGHTER_STATUS_KIND_ATTACK_S3,
                    *FIGHTER_STATUS_KIND_ATTACK_HI3,
                    *FIGHTER_STATUS_KIND_ATTACK_LW3,
                    *FIGHTER_STATUS_KIND_ATTACK_DASH,
                    //smashes
                    *FIGHTER_STATUS_KIND_ATTACK_S4_START,
                    *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
                    *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
                    *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
                    *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
                    *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
                    *FIGHTER_STATUS_KIND_ATTACK_S4,
                    *FIGHTER_STATUS_KIND_ATTACK_HI4,
                    *FIGHTER_STATUS_KIND_ATTACK_LW4,
                    //special-n
                    *FIGHTER_STATUS_KIND_SPECIAL_N,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX,
                    //special-s
                    *FIGHTER_STATUS_KIND_SPECIAL_S,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK,
                    //special-hi
                    *FIGHTER_STATUS_KIND_SPECIAL_HI,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4,
                    //special-lw
                    *FIGHTER_STATUS_KIND_SPECIAL_LW,
                    *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT,
                    //other
                    *FIGHTER_STATUS_KIND_ATTACK_AIR,
                    *FIGHTER_STATUS_KIND_LADDER_ATTACK,
                    *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
                    *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
                    *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK
                ].contains(&status_kind) {
                    VarModule::set_int(agent.module_accessor, instance::IKE_INT_STORED_AETHER_EFFECT_COUNT, param::IKE_INT_STORED_AETHER_EFFECT_FRAME);
                }
                else if [//hammar
                    *FIGHTER_STATUS_KIND_HAMMER_FALL,
                    *FIGHTER_STATUS_KIND_HAMMER_JUMP,
                    *FIGHTER_STATUS_KIND_HAMMER_TURN,
                    *FIGHTER_STATUS_KIND_HAMMER_WAIT,
                    *FIGHTER_STATUS_KIND_HAMMER_WALK,
                    *FIGHTER_STATUS_KIND_HAMMER_LANDING,
                    *FIGHTER_STATUS_KIND_HAMMER_JUMP_SQUAT,
                    //swallowed
                    *FIGHTER_STATUS_KIND_SWALLOWED,
                    *FIGHTER_STATUS_KIND_SWALLOWED_DRINK,
                    *FIGHTER_STATUS_KIND_SWALLOWED_CANCEL,
                    *FIGHTER_STATUS_KIND_SWALLOWED_THROWN,
                    *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE,
                    *FIGHTER_STATUS_KIND_SWALLOWED_CANCELED,
                    *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR
                ].contains(&status_kind) 
                || ItemModule::get_have_item_kind(agent.module_accessor, 0) == *ITEM_KIND_GENESISSET {
                    VarModule::set_int(agent.module_accessor, instance::IKE_INT_STORED_AETHER_EFFECT_COUNT, param::IKE_INT_STORED_AETHER_EFFECT_FRAME);
                    VarModule::on_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER_EFFECT_DISABLE);
                    stored_aether_effect_off(agent);
                }else if VarModule::get_int(agent.module_accessor, instance::IKE_INT_STORED_AETHER_EFFECT_COUNT) <= 0 {
                    VarModule::set_int(agent.module_accessor, instance::IKE_INT_STORED_AETHER_EFFECT_COUNT, param::IKE_INT_STORED_AETHER_EFFECT_FRAME);
                    if sv_math::rand(hash40("fighter"), 3) == 0 {
                        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_magicball_aura"), Hash40::new("sword"), 0, 5, 0, 0, 0, 0, 0.6, false);
                    }
                    if sv_math::rand(hash40("fighter"), 3) == 0 {
                        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_magicball_aura"), Hash40::new("sword"), 0, 9, 0, 0, 0, 0, 0.6, false);
                    }
                    if sv_math::rand(hash40("fighter"), 3) == 0 {
                        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_magicball_aura"), Hash40::new("sword"), 0, 13, 0, 0, 0, 0, 0.6, false);
                    }
                }else {
                    VarModule::dec_int(agent.module_accessor, instance::IKE_INT_STORED_AETHER_EFFECT_COUNT);
                }
            }else if VarModule::is_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER_EFFECT_DISABLE) {
                if [//hammar
                    *FIGHTER_STATUS_KIND_HAMMER_FALL,
                    *FIGHTER_STATUS_KIND_HAMMER_JUMP,
                    *FIGHTER_STATUS_KIND_HAMMER_TURN,
                    *FIGHTER_STATUS_KIND_HAMMER_WAIT,
                    *FIGHTER_STATUS_KIND_HAMMER_WALK,
                    *FIGHTER_STATUS_KIND_HAMMER_LANDING,
                    *FIGHTER_STATUS_KIND_HAMMER_JUMP_SQUAT,
                    //swallowed
                    *FIGHTER_STATUS_KIND_SWALLOWED,
                    *FIGHTER_STATUS_KIND_SWALLOWED_DRINK,
                    *FIGHTER_STATUS_KIND_SWALLOWED_CANCEL,
                    *FIGHTER_STATUS_KIND_SWALLOWED_THROWN,
                    *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE,
                    *FIGHTER_STATUS_KIND_SWALLOWED_CANCELED,
                    *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR
                ].contains(&status_kind) == false 
                && ItemModule::get_have_item_kind(agent.module_accessor, 0) != *ITEM_KIND_GENESISSET {
                    VarModule::off_flag(agent.module_accessor, instance::IKE_FLAG_STORED_AETHER_EFFECT_DISABLE);
                    stored_aether_effect_on(agent);
                }
            }
        }
    }
}

pub mod consts;
mod normals;
mod specials;
// mod others;

pub fn install() {
    let agent = &mut smashline::Agent::new("ike");
    agent.on_line(Main, ike_frame);
    normals::install(agent);
    specials::install(agent);
    // other::install(agent);
    agent.install();
}