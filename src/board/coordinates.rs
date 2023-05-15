use sdl2::rect::Point;
use super::{HEIGHT, WIDTH, HEX_SIZE};
use crate::Direction;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Coordinates {
    pub q: i32,
    pub r: i32,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        return self.q == other.q && self.r == other.r;
    }
}

// Calculate the center point in pixel.
impl std::convert::Into<Point> for Coordinates {
    fn into(self) -> Point {
        const HEX_DIAMETER: f64 = HEX_SIZE * 1.9; // > 1.732 < 2
        const X_RATIO: i32 = (HEX_DIAMETER * 0.5) as i32; // "2 times 0.5 is needed for the logic" WHAT. UPDATE: OH MY GOD IT'S ACTUALLY SO MUCH WORSE. IT'S 2 times -COS(PI * (2 / 3))
        const Y_RATIO: i32 = (HEX_DIAMETER * 0.86602540378) as i32; // SIN(PI * (2/3))
        let y = Y_RATIO * self.r;
        let x = X_RATIO * self.r + (HEX_DIAMETER as i32) * self.q;
        Point::new(x + ((WIDTH / 4) as i32), y + ((HEIGHT / 5) as i32))
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("q: {}, r: {}", self.q, self.r))
    }
}

impl Coordinates {
    pub fn from_offset(x: i32, y: i32) -> Self {
        let r = y;
        let q = x - (y - (y & 1)) / 2;
        Coordinates { q: q, r: r }
    }

    pub fn add_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::TopRight => Self {
                q: self.q + 1,
                r: self.r - 1,
            },
            Direction::Right => Self {
                q: self.q + 1,
                r: self.r,
            },
            Direction::BottomRight => Self {
                q: self.q,
                r: self.r + 1,
            },
            Direction::BottomLeft => Self {
                q: self.q - 1,
                r: self.r + 1,
            },
            Direction::Left => Self {
                q: self.q - 1,
                r: self.r,
            },
            Direction::TopLeft => Self {
                q: self.q,
                r: self.r - 1,
            },
        }
    }
}