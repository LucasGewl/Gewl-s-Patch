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

//Side B End
#[acmd_script(agent = "falco", script = "game_specialairsend", category = ACMD_GAME)]
unsafe fn falco_side_b_end(fighter: &mut L2CAgentBase){
    frame(fighter.lua_state_agent,6.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_FALCO_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL);
    }
    frame(fighter.lua_state_agent,15.0);
    JostleModule::set_status(fighter.module_accessor,true);
    frame(fighter.lua_state_agent,45.0);
    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
}

//Bair
#[acmd_script(agent = "falco", script = "game_attackairb",category = ACMD_GAME)]
unsafe fn falco_bair(fighter: &mut L2CAgentBase){
    frame(fighter.lua_state_agent,1.0);
    macros::FT_MOTION_RATE(fighter, 0.35);

    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,9.0);
    macros::FT_MOTION_RATE(fighter,1.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("kneer"), 13.0, 50, 115, 0, 0, 5.6, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneer"), 13.0, 50, 115, 0, 0, 3.2, -3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneel"), 13.0, 50, 115, 0, 0, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent,12.0);
    macros::FT_MOTION_RATE(fighter,2.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("kneer"), 7.0, 60, 75, 0, 35, 3.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneer"), 7.0, 60, 75, 0, 35, 2.8, -2.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,2, 0, Hash40::new("kneel"), 7.0, 60, 75, 0, 35, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    
    frame(fighter.lua_state_agent,18.0);
    macros::FT_MOTION_RATE(fighter,1.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair (I know this guy!!! Falco Edition)
#[acmd_script(agent = "falco", script = "game_attackairlw",category = ACMD_GAME)]
unsafe fn falco_dair(fighter: &mut L2CAgentBase){
    frame(fighter.lua_state_agent,1.0);
    macros::FT_MOTION_RATE(fighter, 0.35);
    frame(fighter.lua_state_agent,4.0);
    if macros::is_excute(fighter){
      WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,10.0);
    macros::FT_MOTION_RATE(fighter, 1);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("kneel"), 9.6, 285, 80, 0, 30, 6.4, 3.1, 0.0, 1.0, None, None, None, 1.3, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneel"), 9.6, 285, 80, 0, 30, 4.2, 0.0, 3.2, 1.0, None, None, None, 1.3, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //macros::ATTACK(fighter,1, 0, Hash40::new("kneel"), 13.0, 80, 50, 0, 55, 4.2, 4.2, 0.0, 1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    frame(fighter.lua_state_agent,22.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("kneel"), 7.2, 285, 80, 0, 15, 5.3, 3.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter,1, 0, Hash40::new("kneel"), 7.2, 285, 80, 0, 15, 4.2, 0.0, 3.2, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //macros::ATTACK(fighter,1, 0, Hash40::new("kneel"), 8.0, 80, 40, 0, 30, 4.2, 4.2, 0.0, 1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

    }    
    frame(fighter.lua_state_agent,31.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,37.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,38.0);
    macros::FT_MOTION_RATE(fighter,2);
    frame(fighter.lua_state_agent,43.0);
    macros::FT_MOTION_RATE(fighter,1);
}

pub fn install() {
    smashline::install_acmd_scripts!(
        falco_dair,
        falco_bair,
        falco_side_b_end
    );
}