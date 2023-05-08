extern crate sdl2;
use crate::direction::Direction;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

const HEX_SIZE: f64 = 30.0;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

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

// Used to calculate the center point in pixel.
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
    pub fn from_offset(x: i32, y: i32) -> Coordinates {
        let r = y;
        let q = x - (y - (y & 1)) / 2;
        Coordinates { q: q, r: r }
    }

    pub fn add_direction(&self, direction: Direction) -> Coordinates {
        match direction {
            Direction::TopRight => Coordinates {
                q: self.q + 1,
                r: self.r - 1,
            },
            Direction::Right => Coordinates {
                q: self.q + 1,
                r: self.r,
            },
            Direction::BottomRight => Coordinates {
                q: self.q,
                r: self.r + 1,
            },
            Direction::BottomLeft => Coordinates {
                q: self.q - 1,
                r: self.r + 1,
            },
            Direction::Left => Coordinates {
                q: self.q - 1,
                r: self.r,
            },
            Direction::TopLeft => Coordinates {
                q: self.q,
                r: self.r - 1,
            },
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    pub coordinates: Coordinates,
    pub free: bool,
}

impl Tile {
    pub fn new(coordinates: Coordinates, free: bool) -> Tile {
        Tile {
            coordinates: coordinates,
            free: free,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let orig: Point = self.coordinates.into();
        let mut points: [Point; 6] = [Point::new(0, 0); 6];
        for i in 0..6 {
            let angle_deg: f64 = (60 * (i as i32) - 30) as f64;
            let angle_rad: f64 = f64::to_radians(angle_deg);
            points[i].x = orig.x + (f64::round(HEX_SIZE * f64::cos(angle_rad)) as i32);
            points[i].y = orig.y + (f64::round(HEX_SIZE * f64::sin(angle_rad)) as i32);
        }
        canvas.set_draw_color(Color::RGB(0, 20, 0));

        let mut color: Color = Color::RGB(170, 170, 170);
        if self.free {
            color = Color::RGB(200, 200, 200)
        }

        canvas
            .filled_polygon(
                &points.map(|p| p.x as i16),
                &points.map(|p| p.y as i16),
                color,
            )
            .unwrap();
        for i in 0..6 {
            canvas
                .aa_line(
                    points[i].x as i16,
                    points[i].y as i16,
                    points[(i + 1) % 6].x as i16,
                    points[(i + 1) % 6].y as i16,
                    Color::RGB(0, 20, 0),
                )
                .unwrap();
        }
        /*
        canvas
            .string(
                (orig.x - 16) as i16,
                (orig.y - 15) as i16,
                &format!("{}", self.coordinates.q).to_string(),
                Color::RGB(0, 20, 0),
            )
            .unwrap();
        canvas
            .string(
                (orig.x + 11) as i16,
                (orig.y - 4) as i16,
                &format!("{}", self.coordinates.r).to_string(),
                Color::RGB(0, 20, 0),
            )
            .unwrap();
        */
    }
}
