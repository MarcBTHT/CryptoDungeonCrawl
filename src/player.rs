use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player { //Methodes associée à la structure Player
    pub fn new(position: Point) -> Self {
        Self { position }
    }
    // Render the player
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
    // Update the player (move)
    pub fn update(&mut self, ctx:&mut BTerm,map : &Map){
        // We check for a key press (store in delta)
        if let Some(key)= ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right=> Point::new(1, 0),
                VirtualKeyCode::Up=> Point::new(0,-1),
                VirtualKeyCode::Down=> Point::new(0, 1),
                _=> Point::zero()
            };
            //Player's new position (is the current position + delta)
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) { //Look if the move is valid
                self.position= new_position;
            }
        }
    }

}



