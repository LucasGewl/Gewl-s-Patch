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

//Shine Grounded
#[acmd_script(agent = "wolf", script = "game_speciallwstart",category = ACMD_GAME)]
unsafe fn wolf_shine_start_ground(fighter: &mut L2CAgentBase){

    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("top"),6.4,85,60,0,75,9.0,0.0,7.0,0.0,None,None,None,0.4,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_ENERGY)
    }
    frame(fighter.lua_state_agent,5.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Shine Airborne
#[acmd_script(agent = "wolf", script = "game_specialairlwstart",category = ACMD_GAME)]
unsafe fn wolf_shine_start_air(fighter: &mut L2CAgentBase){

    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("top"),6.4,85,60,0,75,9.0,0.0,7.0,0.0,None,None,None,0.4,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_ENERGY)
    }
    frame(fighter.lua_state_agent,5.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Up Air
#[acmd_script(agent = "wolf", script = "game_attackairhi",category = ACMD_GAME)]
unsafe fn wolf_uair(fighter: &mut L2CAgentBase){

    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,7.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("handr"),8.8,80,90,0,30,5.0,2.0,0.5,0.2,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1,0,Hash40::new("armr"),8.8,80,90,0,30,4.0,1.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,2,0,Hash40::new("shoulderr"),8.8,80,90,0,30,4.0,1.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent,8.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("handr"),12.0,80,90,0,30,5.0,2.0,0.5,0.2,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent,9.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("handr"),8.8,80,90,0,30,5.0,2.0,0.5,0.2,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,31.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair (I know this guy!!!)
#[acmd_script(agent = "wolf", script = "game_attackairlw",category = ACMD_GAME)]
unsafe fn wolf_dair(fighter: &mut L2CAgentBase){

    frame(fighter.lua_state_agent,5.0);
    if macros::is_excute(fighter){
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,16.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("top"), 10.4, 270, 90, 0, 6, 5.0, 0.0, 6.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 12.0, 270, 90, 0, 6, 7.0, 0.0, 2.0, 0.5, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    
    frame(fighter.lua_state_agent,17.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,36.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Up Throw
#[acmd_script( agent = "wolf", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn wolf_uthrow(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0,6.0,90,60,0,65,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0,3.0,361,100,0,60,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
    }
    
    frame(fighter.lua_state_agent,26.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("hip"),1.0,55,100,0,50,6.0,11.0,0.0,0.0,None,None,None,1.1,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(fighter.module_accessor,true,false);
        macros::CHECK_FINISH_CAMERA(fighter,6,20);
    }
    frame(fighter.lua_state_agent,27.0);
    if macros::is_excute(fighter){
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target,target_group,target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 0.8)
}


pub fn install() {
    smashline::install_acmd_scripts!(
        wolf_shine_start_ground,
        wolf_shine_start_air,
        wolf_uair,
        wolf_uthrow,
        wolf_dair
    );
}
