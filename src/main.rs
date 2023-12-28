mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}
use prelude::*;
use std::time::{Duration, Instant};
// I don't use game loop because :
// Because if I need to handle the player's input, while I wait to refresh ...
//
struct State {
    // We add the map to the state
    map: Map,
    player: Player,
    last_update: Instant,
}

impl State {
    //We initialize the State's constructor !!!!
    fn new() -> Self {
        let mut rng =RandomNumberGenerator::new();
        let map_builder=MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            last_update: Instant::now(), // Initialisation de la variable de temps
        }
    }
    // Méthode pour déplacer le joueur (TEST POUR SAVOIR SI LA DYNAMIQUE FONCTIONNE)
    fn move_player(&mut self) {
        let delta = Point::new(1, 0); // Déplacer le joueur d'une case vers la droite
        let new_position = self.player.position + delta;
        if self.map.can_enter_tile(new_position) {
            self.player.position = new_position;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let current_time = Instant::now();
        if current_time.duration_since(self.last_update) >= Duration::from_millis(500) {
            // Mise à jour toutes les 500 millisecondes
            // NEED TO DEFINE THE CODE TO MOVE THE MONSTERS HERE
            self.move_player(); //Test

            self.last_update = current_time; // Réinitialisation du compteur de temps
        }

        ctx.cls(); //Clear the screen
        self.map.render(ctx); //Call the map's render function
        self.player.render(ctx); //Call the player's render function
        self.player.update(ctx, &self.map); //Call the player's update function
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("DungeonCrawler")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
