use super::{HEIGHT, HEX_SIZE, WIDTH};
use crate::Direction;
use std::ops::{Add, Sub};

use sdl2::rect::Point;

/// Tweak value between 1.732 and 2 to add or remove gap.
const HEX_DIAMETER: f64 = HEX_SIZE * 1.9;

/// Contains axial coordinates in a q/r form.
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

/// COS(PI / 3)
const M11: f64 = 0.5;
/// COS(0)
const M12: f64 = 1.0;
/// SIN(PI / 3)
const M21: f64 = 0.86602540378;
/// SIN(0)
const M22: f64 = 0.0;

impl Into<Point> for Coordinates {
    /// Calculate the center point in pixel.
    fn into(self) -> Point {
        let r = self.r as f64;
        let q = self.q as f64;
        let x = (HEX_DIAMETER * (M11 * r + M12 * q)) as i32;
        let y = (HEX_DIAMETER * (M21 * r + M22 * q)) as i32;
        Point::new(x + ((WIDTH / 4) as i32), y + ((HEIGHT / 5) as i32))
    }
}

const DET_M: f64 = M11 * M22 - M12 * M21;
/// constant so we know it's non null.

const N11: f64 = (1.0 / DET_M) * M22;
const N12: f64 = (1.0 / DET_M) * -M12;
const N21: f64 = (1.0 / DET_M) * -M21;
const N22: f64 = (1.0 / DET_M) * M11;

impl From<Point> for Coordinates {
    /// Compute [Coordinates] from pixel position.
    fn from(point: Point) -> Self {
        let x = f64::from(point.x) - f64::from(WIDTH) / 4.0;
        let y = f64::from(point.y) - f64::from(HEIGHT) / 5.0;
        let coords: FloatCoordinates = FloatCoordinates {
            q: (1.0 / HEX_DIAMETER) * (N21 * x + N22 * y),
            r: (1.0 / HEX_DIAMETER) * (N11 * x + N12 * y),
        };
        Coordinates::round(coords)
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("q: {}, r: {}", self.q, self.r))
    }
}

impl Add<Direction> for Coordinates {
    type Output = Coordinates;
    /// Compute coordinate if the next step is in given direction.
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

impl Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Self::Output {
        Coordinates {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

/// Like Coordinates but used for trajectory computations.
/// You might *think* you may need it, but you don't.
/// Do not use.
#[derive(Clone, Copy, Debug)]
pub struct FloatCoordinates {
    pub q: f64,
    pub r: f64,
}

/// Very small coordinate used to nudge a line in a direction or another.
const COORDINATES_EPSILON: FloatCoordinates = FloatCoordinates { q: 1e-6, r: 2e-6 };

impl Add<FloatCoordinates> for FloatCoordinates {
    type Output = FloatCoordinates;

    fn add(self, rhs: FloatCoordinates) -> Self::Output {
        FloatCoordinates {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub<FloatCoordinates> for FloatCoordinates {
    type Output = FloatCoordinates;

    fn sub(self, rhs: FloatCoordinates) -> Self::Output {
        FloatCoordinates {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

/// Weird maths for line drawing between two points.
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Weird maths for line drawing between two axial coordinates.
fn axial_lerp(a: FloatCoordinates, b: FloatCoordinates, t: f64) -> FloatCoordinates {
    FloatCoordinates {
        q: lerp(a.q, b.q, t),
        r: lerp(a.r, b.r, t),
    }
}

impl Coordinates {
    /// Transforms x/y coordinates into cube coordinates.
    pub fn from_offset(x: i32, y: i32) -> Self {
        let r = y;
        let q = x - (y - (y & 1)) / 2;
        Coordinates { q: q, r: r }
    }

    /// More precise coordinate round.
    pub fn round(coords: FloatCoordinates) -> Self {
        let qgrid = coords.q.round();
        let rgrid = coords.r.round();

        let qremainder = coords.q - qgrid;
        let rremainder = coords.r - rgrid;

        if qremainder.abs() >= rremainder.abs() {
            return Coordinates {
                q: (qgrid + (qremainder + 0.5 * rremainder).round()) as i32,
                r: rgrid as i32,
            };
        }
        return Coordinates {
            q: qgrid as i32,
            r: (rgrid + (rremainder + 0.5 * qremainder).round()) as i32,
        };
    }

    /// Manhattan distance between two hexes.
    /// Coordinates don't have to exist on the board.
    pub fn distance(self, target: Coordinates) -> u32 {
        let vec: Coordinates = self - target;
        let manhattan = vec.q.abs() + (vec.q.abs() + vec.r.abs()) + vec.r.abs();
        (manhattan / 2) as u32
    }

    /// Returns one or two lines (one with +epsilon, one with -epsilon) between two hexes.
    /// Coordinates don't have to exist on the board.
    pub fn line(self, target: Coordinates) -> (Vec<Coordinates>, Vec<Coordinates>) {
        let mut distance = self.distance(target);
        if distance == 0 {
            distance = 1;
        }

        let start_float = FloatCoordinates {
            q: self.q as f64,
            r: self.r as f64,
        };
        let end_float = FloatCoordinates {
            q: target.q as f64,
            r: target.r as f64,
        };

        let mut first: Vec<Coordinates> = Vec::with_capacity(distance as usize);
        let mut second: Vec<Coordinates> = Vec::with_capacity(distance as usize);

        for i in 1..(distance) {
            let plus = axial_lerp(
                start_float + COORDINATES_EPSILON,
                end_float,
                1.0 / distance as f64 * i as f64,
            );
            let minus = axial_lerp(
                start_float - COORDINATES_EPSILON,
                end_float,
                1.0 / distance as f64 * i as f64,
            );
            first.push(Coordinates::round(plus));
            second.push(Coordinates::round(minus))
        }

        (first, second)
    }
}
