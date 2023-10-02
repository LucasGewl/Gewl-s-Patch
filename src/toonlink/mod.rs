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


//Dash attack
#[acmd_script(agent = "toonlink", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn toonlink_dashattack(fighter: &mut L2CAgentBase){
macros::FT_MOTION_RATE(fighter,0.7);
frame(fighter.lua_state_agent,8.0);
macros::FT_MOTION_RATE(fighter,1.1);
if macros::is_excute(fighter){
    macros::ATTACK(fighter,3, 0, Hash40::new("sword2"), 8.0, 20, 70, 0, 70, 4.2, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    macros::ATTACK(fighter,0, 0, Hash40::new("sword2"), 8.0, 20, 70, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    macros::ATTACK(fighter,1, 0, Hash40::new("arml"), 6.0, 20, 70, 0, 70, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
}
frame(fighter.lua_state_agent,10.0);
macros::FT_MOTION_RATE(fighter,0.619);
if macros::is_excute(fighter){
    AttackModule::clear_all(fighter.module_accessor);
}
frame(fighter.lua_state_agent,43.0);
macros::FT_MOTION_RATE(fighter,1);
}

//Bair
#[acmd_script(agent = "toonlink", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn toonlink_bair(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("sword2"), 8.0, 75, 117, 0, 24, 4.0, 6.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("sword2"), 8.0, 75, 117, 0, 24, 5.0, 2.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,2, 0, Hash40::new("arml"), 8.0, 75, 117, 0, 24, 4.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent,2.0);
        if macros::is_excute(fighter){
            AttackModule::clear_all(fighter.module_accessor);
            macros::FT_MOTION_RATE(fighter,1.67);
        }
    frame(fighter.lua_state_agent,24.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
}

//Down Throw
#[acmd_script(agent = "toonlink", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn toonlink_dthrow(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0,4.0,110,110,0,60,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0,3.0,361,100,0,40,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent,22.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("top"),3.0,361,150,0,60,50,0.0,2.4,7.8,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_ELBOW);
        AttackModule::set_catch_only_all(fighter.module_accessor,true,false);
}
    frame(fighter.lua_state_agent,23.0);
    if macros::is_excute(fighter){
        macros::CHECK_FINISH_CAMERA(fighter,-6,4);
        }
    frame(fighter.lua_state_agent,24.0);
    if macros::is_excute(fighter){
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target,target_group,target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}



pub fn install(){
    smashline::install_acmd_scripts!(
        toonlink_dashattack,
        toonlink_bair,
        toonlink_dthrow
    );
}