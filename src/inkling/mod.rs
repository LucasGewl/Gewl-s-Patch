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



//Up Air
#[acmd_script( agent = "inkling", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn inkling_uair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent,2.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,11.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("kneer"),5.0,367,20,0,30,4.0,-1.0,-0.5,-0.8,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1,0,Hash40::new("kneer"),5.0,367,20,0,15,3.6,4.2,-0.5,-0.8,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,2,0,Hash40::new("kneer"),5.0,367,20,0,15,2.6,2.4,-6.0,1.9,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent,15.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,17.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0,0,Hash40::new("kneel"),10.5,90,105,0,30,4.5,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1,0,Hash40::new("kneel"),10.5,90,105,0,30,4.5,5.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0,0.0,0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK,*ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent,20.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,40.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        inkling_uair
    );
}