use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, RenderTarget};

use super::HEX_SIZE;
use crate::Coordinates;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub free: bool,
}

impl Tile {
    pub fn new(free: bool) -> Self {
        Tile { free: free }
    }

    pub fn draw<T>(&self, canvas: &mut Canvas<T>, coordinates: Coordinates, debug: bool)
    where
        T: RenderTarget,
    {
        let orig: Point = coordinates.into();
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
        if debug {
            canvas
                .string(
                    (orig.x - 16) as i16,
                    (orig.y - 15) as i16,
                    &format!("{}", coordinates.q).to_string(),
                    Color::RGB(0, 20, 0),
                )
                .unwrap();
            canvas
                .string(
                    (orig.x + 11) as i16,
                    (orig.y - 4) as i16,
                    &format!("{}", coordinates.r).to_string(),
                    Color::RGB(0, 20, 0),
                )
                .unwrap();
        }
    }
}
