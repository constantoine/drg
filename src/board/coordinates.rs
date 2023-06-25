use super::{HEIGHT, HEX_SIZE, WIDTH};
use crate::Direction;
use sdl2::rect::Point;

const HEX_DIAMETER: f64 = HEX_SIZE * 1.9; // > 1.732 < 2

#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct Coordinates {
    pub q: i32,
    pub r: i32,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        return self.q == other.q && self.r == other.r;
    }
}

const M11: f64 = 0.5; //COS(PI / 3)
const M12: f64 = 1.0; //COS(0)
const M21: f64 = 0.86602540378; //SIN(PI / 3)
const M22: f64 = 0.0; //SIN(0)

// Calculate the center point in pixel.
impl std::convert::Into<Point> for Coordinates {
    fn into(self) -> Point {
        let r = self.r as f64;
        let q = self.q as f64;
        let x = (HEX_DIAMETER * (M11 * r + M12 * q)) as i32;
        let y = (HEX_DIAMETER * (M21 * r + M22 * q)) as i32;
        Point::new(x + ((WIDTH / 4) as i32), y + ((HEIGHT / 5) as i32))
    }
}

const DET_M: f64 = M11 * M22 - M12 * M21; //constant so we know it's non null;

const N11: f64 = (1.0 / DET_M) * M22;
const N12: f64 = (1.0 / DET_M) * -M12;
const N21: f64 = (1.0 / DET_M) * -M21;
const N22: f64 = (1.0 / DET_M) * M11;

// Calculate the center point in pixel.
impl std::convert::From<Point> for Coordinates {
    fn from(point: Point) -> Self {
        let x = f64::from(point.x) - f64::from(WIDTH) / 4.0;
        let y = f64::from(point.y) - f64::from(HEIGHT) / 5.0;
        let r = ((1.0 / HEX_DIAMETER) * (N11 * x + N12 * y)).round() as i32;
        let q = ((1.0 / HEX_DIAMETER) * (N21 * x + N22 * y)).round() as i32;
        Self { q: q, r: r }
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("q: {}, r: {}", self.q, self.r))
    }
}

impl std::ops::Add<Direction> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
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

impl std::ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl std::ops::Sub<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl Coordinates {
    pub fn from_offset(x: i32, y: i32) -> Self {
        let r = y;
        let q = x - (y - (y & 1)) / 2;
        Coordinates { q: q, r: r }
    }

    pub fn distance(self, target: Coordinates) -> u32 {
        let vec: Coordinates = self - target;
        let manhattan = vec.q.abs() + (vec.q.abs() + vec.r.abs()) + vec.r.abs();
        (manhattan / 2) as u32
    }
}
