use std::collections::HashMap;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

use super::{coordinates::Coordinates, tile::Tile};

/// A game board containing a reference to each of its [Tiles][Tile].
#[derive(Debug)]
pub struct Board {
    tiles: HashMap<Coordinates, Tile>,
}

impl Board {
    /// Initialize a new board with
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();
        let mut tiles: HashMap<Coordinates, Tile> = HashMap::new();
        for x in 0..15 {
            for y in 0..15 {
                // remove last piece from every odd row to get nice square board.
                if x == 14 && y & 1 != 0 {
                    continue;
                }
                // Remove pieces to be iso with the canonical board.
                if (x == 0 || x == 14) && (y == 4 || y == 10) {
                    continue;
                }

                // Generated tiles have 8% chance of being obstacles.
                let mut free = rng.gen_bool(0.92);
                if x == 0 && y == 0 {
                    free = false;
                }

                tiles.insert(Coordinates::from_offset(x, y), Tile::new(free));
            }
        }
        Board { tiles: tiles }
    }

    /// Iterate over every tile and draw its base.
    pub fn draw<T>(&self, canvas: &mut Canvas<T>)
    where
        T: RenderTarget,
    {
        for (coords, tile) in self.tiles.iter() {
            tile.draw(canvas, *coords, true);
        }
    }

    /// Get a reference to a tile on the board.
    pub fn get(&self, coords: Coordinates) -> Option<&Tile> {
        match self.tiles.get(&coords.clone()) {
            Some(tile) => Some(tile),
            None => None,
        }
    }

    /// Set tile as free on the gameboard.
    pub fn free(&mut self, coords: Coordinates) {
        match self.tiles.get_mut(&coords.clone()) {
            Some(tile) => tile.free = true,
            None => (),
        }
    }

    /// Set a tile as filled on the gameboard.
    pub fn fill(&mut self, coords: Coordinates) {
        match self.tiles.get_mut(&coords.clone()) {
            Some(tile) => tile.free = false,
            None => (),
        }
    }
}
