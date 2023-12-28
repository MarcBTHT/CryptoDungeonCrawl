use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)] //Derive list : Clone makes a deep copy of the object, PartialEq allows us to compare two objects, Copy allows us to copy the object
pub enum TileType {
    //On définit les différents carreaux de la carte
    Wall,  // Represent by # and player by @
    Floor, // Represent by .
}
pub struct Map {
    //on définit la carte (vide pour l'instant)
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    //Vectors are indexed on a single dimension, so we need to convert our 2D coordinates into a single index
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    //Constructor for the Map type
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES], //Creat a map of floor tiles !!FOR NOW!!
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        //The map needs to be able to draw itself to the screen
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y); //Determine the index of the tile we want to draw (0,0)=0, (1,0)=1, (0,1)=80, (1,1)=81 ...
                match self.tiles[idx] {
                    //We render each map tile
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    //////////////// SET the map ////////////////

    // If an x/y coordinate pair is within the bounds of the map.
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
    // To know if the player can enter a tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }
    // We need to be able to check if a point is in bounds of the map
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
