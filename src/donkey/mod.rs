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

//Cargo Up Throw
#[acmd_script(agent = "donkey", script = "game_throwfhi", category = ACMD_GAME)]
unsafe fn donkey_cupthrow(fighter: &mut L2CAgentBase){
    if macros::is_excute(fighter){
        macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0,12.0,92,41,0,68,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
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
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

pub fn install(){
    smashline::install_acmd_scripts!(
        donkey_cupthrow
    );
}