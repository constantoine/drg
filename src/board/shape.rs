use super::coordinates::Coordinates;

#[derive(Debug, Clone)]
pub struct Shape {
    /// Center of the shape.
    pub center: Coordinates,

    /// Contains the coordinates relative to the center
    /// Can or not contain the center, depending on if the shape is hollow or not.
    pub tiles: Vec<Coordinates>,
}

impl Shape {
    pub fn rotate_clockwise(&mut self) {
        for tile in self.tiles.iter_mut() {
            let (q, r, s) = (tile.q, tile.r, -tile.q - tile.r);
            let vec = (-r, -s, -q);
            (*tile).q = vec.0;
            (*tile).r = vec.1;
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        for tile in self.tiles.iter_mut() {
            let (q, r, s) = (tile.q, tile.r, -tile.q - tile.r);
            let vec = (-s, -q, -r);
            (*tile).q = vec.0;
            (*tile).r = vec.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_clockwise() {
        let mut shape = Shape {
            center: Coordinates { q: -1, r: -2 },
            tiles: vec![
                Coordinates { q: 0, r: -1 },
                Coordinates { q: 1, r: -1 },
                Coordinates { q: -1, r: 2 },
            ],
        };

        shape.rotate_clockwise();

        assert_eq!(
            shape.tiles,
            vec![
                Coordinates { q: 1, r: -1 },
                Coordinates { q: 1, r: 0 },
                Coordinates { q: -2, r: 1 }
            ]
        );
    }

    #[test]
    fn rotate_counterclockwise() {
        let mut shape = Shape {
            center: Coordinates { q: -1, r: -2 },
            tiles: vec![
                Coordinates { q: 0, r: -1 },
                Coordinates { q: 1, r: -1 },
                Coordinates { q: -1, r: 2 },
            ],
        };

        shape.rotate_counterclockwise();

        assert_eq!(
            shape.tiles,
            vec![
                Coordinates { q: -1, r: 0 },
                Coordinates { q: 0, r: -1 },
                Coordinates { q: 1, r: 1 }
            ]
        );
    }
}
