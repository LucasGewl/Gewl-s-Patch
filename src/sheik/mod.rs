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

//Jab 1

#[acmd_script( agent = "sheik", script = "game_attack11", category = ACMD_GAME )]
unsafe fn sheik_jab1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("top"),4.0,75,45,0,35,1.7,0.0,9.0,2.0,None,None,None,1.6,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1,0,Hash40::new("top"),4.0,75,45,0,35,1.7,0.0,9.0,5.0,None,None,None,1.6,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,2,0,Hash40::new("top"),4.0,75,45,0,35,2.0,0.0,8.0,8.2,None,None,None,1.6,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,3,0,Hash40::new("top"),4.0,75,45,0,35,2.0,0.0,8.0,8.2,None,None,None,1.6,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent,4.0);
    if macros:: is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Forward Tilt
#[acmd_script( agent = "sheik", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn sheik_ftilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 5.0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("legr"), 5.6, 84, 100, 0, 30, 3.8, 0.7, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneer"), 5.6, 86, 100, 0, 30, 3.0, 0.4, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,2, 0, Hash40::new("kneer"), 5.6, 78, 106, 0, 30, 4.0, 4.3, 0.2, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
    }
    frame(fighter.lua_state_agent,8.0);
    if macros:: is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 3.0);
    }
}

//Down Tilt
#[acmd_script( agent = "sheik", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn sheik_dtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,1, 0, Hash40::new("legr"), 6.4, 70, 75, 0, 37, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,0, 0, Hash40::new("legr"), 6.4, 90, 75, 0, 37, 4.2, 4.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,2, 0, Hash40::new("kneer"), 6.4, 90, 75, 0, 37, 4.2, 4.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
    
    }
    frame(fighter.lua_state_agent,6.0);
    if macros:: is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Dash Attack

#[acmd_script( agent = "sheik", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn sheik_dashattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("top"),10.0,85,90,0,40,4.0,0.0,6.0,12.0,None,None,None,1.3,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1,0,Hash40::new("top"),10.0,85,90,0,40,3.5,0.0,5.5,9.0,None,None,None,1.3,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,2,0,Hash40::new("top"),10.0,85,90,0,40,3.0,0.0,5.0,5.0,None,None,None,1.3,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("top"),6.0,361,100,0,20,4.0,0.0,6.0,9.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1,0,Hash40::new("top"),6.0,361,100,0,20,3.0,0.0,5.5,6.5,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,2,0,Hash40::new("top"),6.0,361,100,0,20,2.0,0.0,5.0,4.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,1,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
    }
        frame(fighter.lua_state_agent,12.0);
    if macros:: is_excute(fighter){
            AttackModule::clear_all(fighter.module_accessor);
    }
}

//Forward Air

#[acmd_script( agent = "sheik", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn sheik_fair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,5.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("shoulderl"),10.4,25,80,0,35,5,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1,0,Hash40::new("shoulderl"),10.4,25,80,0,35,5,2.5,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent,6.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("shoulderl"),10.4,25,80,0,35,5,0.0,-0.5,1.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter,1,0,Hash40::new("shoulderl"),10.4,25,80,0,35,5,2.5,-1.0,1.5,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent,7.0);
    if macros:: is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,29.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Up Air

#[acmd_script( agent = "sheik", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn sheik_uair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("footr"),12.0,85,90,0,30,5.0,5.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1,0,Hash40::new("footr"),12.0,85,90,0,30,5.0,5.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent,8.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("footr"),9.0,85,80,0,10,4.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1,0,Hash40::new("footr"),9.0,85,80,0,10,4.0,3.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent,24.0);
    if macros:: is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,40.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Down Throw

#[acmd_script( agent = "sheik", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn sheik_dthrow(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros:: is_excute(fighter){
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 105, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 3.0, 361, 0, 100, 10, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 3.0, 361, 0, 50, 10, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::CHECK_FINISH_CAMERA(fighter, -1, 0);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        }
        macros::FT_MOTION_RATE(fighter, 18.0 / 23.0);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        sheik_jab1,
        sheik_dashattack,
        sheik_fair,
        sheik_uair,
        sheik_dthrow,
        sheik_ftilt,
        sheik_dtilt
    );
}
