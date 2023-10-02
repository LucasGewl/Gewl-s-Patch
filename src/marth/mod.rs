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

//Fair
#[acmd_script(agent = "marth", script = "game_attackairf",category = ACMD_GAME)]
unsafe fn marth_fair(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,6.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("sword1"), 10.4, 63, 80, 0, 40, 3.0, 1.0, 0.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("armr"), 7.2, 43 , 80, 0, 40, 3.8, 1.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,2, 0, Hash40::new("sword1"), 7.2, 43, 80, 0, 40, 3.0, 1.0, 0.0, 2.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent,10.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,33.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

//Dair (Holy Shit I know that guy!)
#[acmd_script(agent = "marth", script = "game_attackairlw",category = ACMD_GAME)]
unsafe fn marth_dair(fighter:&mut L2CAgentBase){
    frame(fighter.lua_state_agent,3.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,9.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,2, 0, Hash40::new("armr"), 7.2, 361, 70, 0, 30, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("sword1"), 7.2, 80, 70, 0, 40, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,0, 0, Hash40::new("sword1"), 10.4, 290, 80, 0, 20, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        //macros::ATTACK(fighter,0, 0, Hash40::new("top"), 15.0, 260, 80, 0, 20, 5.0, 0.0, 3.3, 3.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }

    frame(fighter.lua_state_agent,14.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,55.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Up Air
#[acmd_script(agent = "marth", script = "game_attackairhi",category = ACMD_GAME)]
unsafe fn marth_uair(fighter:&mut L2CAgentBase){
    frame(fighter.lua_state_agent,3.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,5.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,3, 0, Hash40::new("sword1"), 7.2, 80, 80, 0, 40, 3.5, 1.0, 0.0, 0.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("armr"), 7.2, 80, 80, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,2, 0, Hash40::new("colonells"), 7.2, 80, 80, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,0, 0, Hash40::new("sword1"), 10.4, 90, 84, 0, 40, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }

    frame(fighter.lua_state_agent,9.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,27.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair
#[acmd_script(agent = "marth", script = "game_attackairn",category = ACMD_GAME)]
unsafe fn marth_nair(fighter:&mut L2CAgentBase){
    frame(fighter.lua_state_agent,2.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent,6.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("sword1"), 3.5, 75, 50, 0, 45, 3.8, 1.0, 1.3, 1.6, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("armr"), 3.5, 80, 50, 0, 45, 4.0, 1.5, 1.0, 1.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,2, 0, Hash40::new("sword1"), 5.0, 90, 50, 0, 35, 3.3, 1.0, 1.3, 7.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }

    frame(fighter.lua_state_agent,7.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent,15.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,2, 0, Hash40::new("sword1"), 8.0, 50, 100, 0, 50, 3.8, 1.2, 1.1, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("armr"), 8.0, 50, 100, 0, 50, 4.0, 2.0, 1.0, 1.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,0, 0, Hash40::new("sword1"), 8.0, 50, 100, 0, 60, 3.4, 0.8, 1.1, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false,false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }

    frame(fighter.lua_state_agent,22.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent,25.0);
    if macros::is_excute(fighter){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Down Tilt
#[acmd_script(agent = "marth", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn marth_dtilt(fighter: &mut L2CAgentBase){
    frame(fighter.lua_state_agent,7.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,0, 0, Hash40::new("sword1"), 8.0, 30, 40, 0, 50, 2.7, 0.0, 0.0, 8.2, Some(0.0), Some(0.0), Some(8.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 6.4, 30, 40, 0, 40, 2.7, 0.0, 2.7, 15.7, Some(0.0), Some(4.4), Some(9.2), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor,smash::app::AttackHeight(*ATTACK_HEIGHT_LOW),false)
    }
    frame(fighter.lua_state_agent,10.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,20.0);
    if macros::is_excute(fighter){
    CancelModule::enable_cancel(fighter.module_accessor);
    }
}

//Forward Smash
#[acmd_script(agent = "marth", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn marth_fsmash(fighter: &mut L2CAgentBase){
    frame(fighter.lua_state_agent,3.0);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent,10.0);
    if macros::is_excute(fighter){
        macros::ATTACK(fighter,3, 0, Hash40::new("sword1"), 13.0, 361, 75, 0, 48, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,1, 0, Hash40::new("armr"), 13.0, 361, 75, 0, 48, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,2, 0, Hash40::new("bust"), 13.0, 361, 75, 0, 48, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter,0, 0, Hash40::new("sword1"), 18.0, 361, 80, 0, 80, 4.0, 1.0, 0.0, 7.3, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor,smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH),false)
    }
    frame(fighter.lua_state_agent,14.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent,49.0);
    if macros::is_excute(fighter){
    CancelModule::enable_cancel(fighter.module_accessor);
    }
}


//Up Throw
#[acmd_script(agent = "marth", script = "game_throwhi",category = ACMD_GAME)]
unsafe fn marth_uthrow(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0,6.0,90,65,0,61,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0,3.0,361,100,0,40,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent,12.0);
        if macros::is_excute(fighter){
            macros::CHECK_FINISH_CAMERA(fighter, 1, 21)
        }
    frame(fighter.lua_state_agent,13.0);
    if macros::is_excute(fighter){
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent,33.0);
    if macros::is_excute(fighter){
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        marth_fair,
        marth_dtilt,
        marth_uthrow,
        marth_dair,
        marth_uair,
        marth_nair,
        marth_fsmash
    );
}