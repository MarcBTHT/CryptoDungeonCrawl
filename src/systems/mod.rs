use crate::prelude::*;

mod map_render;
mod entity_render;
mod player_input;
mod collisions;
mod monsters_move;
mod hud;
mod tooltips;

pub fn build_main_scheduler() -> Schedule { //Main schedule pour les moves du joueurs (keyboard)
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}
pub fn build_periodic_scheduler() -> Schedule { //Pour le mouvement périodique des monstres
    Schedule::builder()
        .add_system(monsters_move::move_towards_player_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}