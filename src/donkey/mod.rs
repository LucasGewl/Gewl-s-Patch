use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

//Jab 2
#[acmd_script( agent = "donkey", script = "game_attack12", category = ACMD_GAME)]
unsafe fn donkey_jab2(fighter: &mut L2CAgentBase){
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("handr"), 6.0, 75, 90, 0, 40, 7.0, 1.0, 1.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 6.0, 75, 90, 0, 40, 5.5, 0.0, 9.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,2, 0, Hash40::new("top"), 6.0, 75, 90, 0, 40, 5.0, 0.0, 15.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter){
        StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_WAIT, true);
    }
}

//Cargo Up Throw
#[acmd_script(agent = "donkey", script = "game_throwfhi", category = ACMD_GAME)]
unsafe fn donkey_cupthrow(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0,12.0,92,32,0,75,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0,3.0,361,100,0,40,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent,14.0);
    if macros::is_excute(fighter){
        macros::CHECK_FINISH_CAMERA(fighter,1,31);
    }
    frame(fighter.lua_state_agent,15.0);
    if macros::is_excute(fighter){
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no); 
    }
    frame(fighter.lua_state_agent,25.0);
    if macros::is_excute(fighter){
        StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_WAIT, true);
    }
}

pub fn install(){
    smashline::install_acmd_scripts!(
        donkey_cupthrow,
        donkey_jab2
    );
}