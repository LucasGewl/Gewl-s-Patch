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

//Nair
#[acmd_script( agent = "captain", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn captain_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros:: is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("legr"),4.0,74,25,0,50,6.0,3.2,0.0,0.0,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1,0,Hash40::new("legr"),4.0,68,75,0,65,6.0,3.2,0.0,0.0,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,2,0,Hash40::new("kneer"),4.0,80,25,0,50,4.8,4.2,0.0,0.0,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,3,0,Hash40::new("kneer"),4.0,74,75,0,65,4.8,4.2,0.0,0.0,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,4,0,Hash40::new("hip"),4.0,68,25,0,50,5.2,-5.0,0.0,0.0,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent,8.0);
    if macros:: is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
        frame(fighter.lua_state_agent,13.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("kneel"),9.0,35,80,0,30,5.2,4.4,-0.7,1.0,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1,0,Hash40::new("legl"),9.0,35,80,0,30,6.0,3.2,0.0,1.5,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,2,0,Hash40::new("hip"),9.0,35,80,0,30,5.2,0.0,0.0,1.5,None,None,None,0.75,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
    } 
    frame(fighter.lua_state_agent,22.0);
    if macros:: is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,32.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Up Air
#[acmd_script( agent = "captain", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn captain_uair(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("legl"), 10.0, 45, 90, 0, 20, 4.5, 3.2, 2.1, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneel"), 9.0, 45, 90, 0, 20, 5.0, 6.2, 0.9, 0.4, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("legl"), 9.0, 30, 80, 0, 12, 4.5, 3.2, 2.1, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneel"), 8.0, 30, 80, 0, 12, 5.0, 6.2, 0.9, 0.4, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        captain_nair,
        captain_uair
    );
}
