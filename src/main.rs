mod map;
mod player;

mod prelude{
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}
use prelude::*;

/* */
struct State { // We add the map to the state
    map: Map,
    player: Player,
}

impl State{ //We initialize the State's constructor !!!!
    fn new()->Self{
        Self{ 
            map: Map::new(),
            player: Player::new( //We add the player to the state
                Point::new(SCREEN_WIDTH/2,SCREEN_HEIGHT/2)), 
        }
    }
}

impl GameState for State{
    fn tick(&mut self, ctx:&mut BTerm) {
        ctx.cls(); //Clear the screen
        self.map.render(ctx); //Call the map's render function
        self.player.render(ctx); //Call the player's render function
        self.player.update(ctx,&self.map); //Call the player's update function
    }
}

fn main()-> BError{
    let context=BTermBuilder::simple80x50()
    .with_title("DungeonCrawler")
    .with_fps_cap(30.0)
    .build()?;
    main_loop(context,State::new())
}

