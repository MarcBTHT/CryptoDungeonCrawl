mod map;

mod prelude{
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;}
use prelude::*;

/* */
struct State { // We add the map to the state
    map: Map,
}

impl State{ //We initialize the State's constructor !!!!
    fn new()->Self{
        Self{ map: Map::new() }
    }
}

impl GameState for State{
    fn tick(&mut self, ctx:&mut BTerm) {
        ctx.cls(); //Clear the screen
        self.map.render(ctx); //Call the map's render function
    }
}

fn main()-> BError{
    let context=BTermBuilder::simple80x50()
    .with_title("DungeonCrawler")
    .with_fps_cap(30.0)
    .build()?;
    main_loop(context,State::new())
}

