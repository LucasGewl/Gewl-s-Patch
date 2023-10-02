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

//Back Throw
#[acmd_script( agent = "dedede", script = "game_throwb", category = ACMD_GAME )]
unsafe fn dedede_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter){
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0.0,9.0,60,79,0,60,0.0,1.0,*ATTACK_LR_CHECK_F,0.0,true,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,COLLISION_SOUND_ATTR_NONE,ATTACK_REGION_THROW)
        
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        //szerosuit_upb
    );
}