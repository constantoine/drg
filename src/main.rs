mod hex;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("DRG: The Boardgame", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().unwrap();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        for x in -8..7 {
            for y in -8..7 {
                if x == 6 && y & 1 != 0 {
                    continue;
                }
                if (x == -8 || x == 6) && (y == -4 || y == 2) {
                    continue;
                }
                let tile = hex::Tile::new(hex::Coordinates::from_offset(x, y), true);
                tile.draw(&mut canvas);
                let tile = hex::Tile::new(hex::Coordinates::from_offset(x, y), true);
                tile.draw(&mut canvas);
            }
        }
        /*
                for x in -4..4 {
                    for y in -4..4 {
                        if x == 3 && y&1 == 1 {
                            continue;
                        }
                        let tile = hex::Tile::new(hex::Coordinates::from_offset(x, y), true);
                        tile.draw(&mut canvas);
                    }
                }
        */
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
