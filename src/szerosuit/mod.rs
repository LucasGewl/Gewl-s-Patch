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

//Up B Ground
#[acmd_script(agent = "szerosuit", script = "game_specialhi", category = "ACMD_GAME")]
unsafe fn szerosuit_upb_ground(fighter: &mut L2CAgentBase){
    
}


pub fn install() {
    smashline::install_acmd_scripts!(
        //szerosuit_upb
    );
}