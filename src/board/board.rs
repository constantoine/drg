use std::collections::HashMap;

use crate::board::direction::Direction;
use priority_queue::PriorityQueue;
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

    /// Get a list of all the neighbouring tiles.
    pub fn neighbours(&self, coords: Coordinates) -> Vec<Coordinates> {
        let mut neighbours: Vec<Coordinates> = vec![];

        for direction in [
            Direction::TopRight,
            Direction::Right,
            Direction::BottomRight,
            Direction::BottomLeft,
            Direction::Left,
            Direction::TopLeft,
        ] {
            let target = coords + direction;

            if self.get(target).is_some() {
                neighbours.push(target)
            }
        }

        neighbours
    }

    /// Implementation of A* algo. The heuristic function is just Manhattan distance.
    fn astar_search(
        &self,
        from: Coordinates,
        to: Coordinates,
    ) -> (HashMap<Coordinates, Coordinates>, HashMap<Coordinates, i32>) {
        let mut frontier = PriorityQueue::new();
        frontier.push(from, 0);

        let mut came_from: HashMap<Coordinates, Coordinates> = HashMap::new();
        let mut cost_so_far: HashMap<Coordinates, i32> = HashMap::new();

        cost_so_far.insert(from, 0);

        while !frontier.is_empty() {
            let current = frontier.pop().expect("frontier should not be empty");

            if current.0 == to {
                break;
            }

            for next in self.neighbours(current.0).iter() {
                if !self.get(*next).expect("neighbour should exist").free {
                    continue;
                }

                let new_cost = cost_so_far
                    .get(&current.0)
                    .expect("cost should have been in previous iteration")
                    + 1;
                if cost_so_far.get(next).is_none() || &new_cost < cost_so_far.get(next).unwrap() {
                    cost_so_far.insert(*next, new_cost);
                    frontier.push(*next, next.distance(to));
                    came_from.insert(*next, current.0);
                }
            }
        }

        (came_from, cost_so_far)
    }

    /// Path will call the astar_search function to compute shortest path between from and to.
    ///
    /// If no path was found, returns None.
    pub fn path(&self, from: Coordinates, to: Coordinates) -> Option<Vec<Coordinates>> {
        let (came_from, _cost_so_far) = self.astar_search(from, to);

        let mut current = to;
        let mut path: Vec<Coordinates> = vec![];

        if came_from.get(&to).is_none() {
            return None;
        }

        while current != from {
            path.push(current);
            current = *came_from
                .get(&current)
                .expect("came_from should always contain a value for current");
        }

        path.reverse();
        Some(path)
    }
}
