#![warn(clippy::pedantic)]

mod api;
mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}
use prelude::*;
use std::time::{Duration, Instant}; // For the periodic update for the monsters move

use crate::api::fetch_data;
use api::BitcoinData; // Importe la structure BitcoinData
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::time::sleep;

struct State {
    ecs: World,
    resources: Resources,
    main_schedule: Schedule,     // Schedule principal (Move player)
    periodic_schedule: Schedule, // Schedule pour les mises à jour périodiques (Move monsters)
    last_update: Instant,
    bitcoin_data: Arc<Mutex<BitcoinData>>,
    monster_update_rate: u64,
}

impl State {
    //We initialize the State's constructor !!!!
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        let bitcoin_data = Arc::new(Mutex::new(BitcoinData { price: 0.0 }));
        Self {
            ecs,
            resources,
            main_schedule: build_main_scheduler(),
            periodic_schedule: build_periodic_scheduler(),
            last_update: Instant::now(), 
            bitcoin_data: bitcoin_data,
            monster_update_rate: 1000, // Initialisation de la variable de temps
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let current_time = Instant::now();

        let bitcoin_price = {
            let data = self.bitcoin_data.lock().unwrap();
            data.price
        };

        if bitcoin_price > 42580.0 { //Faudrait le faire avec le %
            self.monster_update_rate = 300; // Plus rapide
        } else {
            self.monster_update_rate = 1000; // Plus lent ou normal
        }

        if current_time.duration_since(self.last_update)
            >= Duration::from_millis(self.monster_update_rate)
        {
            self.periodic_schedule
                .execute(&mut self.ecs, &mut self.resources); //Move of the monsters
            println!("Bitcoin Price: {}", bitcoin_price);
            self.last_update = current_time;
        }

        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2); //Clear the third layer to do the Heads-Up Display
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0); //To display monster info with mouse
        self.resources.insert(Point::from_tuple(ctx.mouse_pos())); //To display monster info with mouse
        self.main_schedule
            .execute(&mut self.ecs, &mut self.resources);
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
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png") //Add of a third layer to do the Heads-Up Display
        .build()?;

    let mut rt = Runtime::new().unwrap();
    let mut state = State::new();
    let bitcoin_data_clone = Arc::clone(&state.bitcoin_data);

    rt.spawn(async move {
        loop {
            match fetch_data().await {
                Ok(price) => {
                    let mut data = bitcoin_data_clone.lock().unwrap();
                    data.price = price;
                }
                Err(e) => eprintln!("Error fetching Bitcoin data: {}", e),
            }
            sleep(Duration::from_secs(60)).await;
        }
    });

    main_loop(context, state)
}
