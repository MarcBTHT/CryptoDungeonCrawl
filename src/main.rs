#![warn(clippy::pedantic)]

mod components;
mod spawner;
mod map;
mod map_builder;
mod systems;
mod camera;

// mod player; To supp

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}
use prelude::*;
use std::time::{Duration, Instant};
// I don't use game loop because :
// Because if I need to handle the player's input, while I wait to refresh ...


struct State {
    ecs : World,
    resources: Resources,
    main_schedule: Schedule, // Schedule principal (Move player)
    periodic_schedule: Schedule, // Schedule pour les mises à jour périodiques (Move monsters)
    last_update: Instant,
}

impl State {
    //We initialize the State's constructor !!!!
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            main_schedule: build_main_scheduler(),
            periodic_schedule: build_periodic_scheduler(),
            last_update: Instant::now(), // Initialisation de la variable de temps
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let current_time = Instant::now();
        if current_time.duration_since(self.last_update) >= Duration::from_millis(1000) { //NEED to modify the time base of the price of BTC
            self.periodic_schedule.execute(&mut self.ecs, &mut self.resources); //Move of the monsters
            self.last_update = current_time;
        }

        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.main_schedule.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}
