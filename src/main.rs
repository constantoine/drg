mod board;
mod creature;
mod dice;
mod utils;

use board::board::Board;
use board::coordinates::Coordinates;
use board::direction::Direction;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    // SDL init.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("DRG: The Boardgame", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().unwrap();
    let manager = sdl2::ttf::init().unwrap();
    let font = manager
        .load_font("/usr/share/fonts/truetype/unifont/unifont.ttf", 22)
        .unwrap();
    canvas.present();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Map generation.
    let mut direction: Direction = Direction::Right;
    let mut location: Coordinates = Coordinates { q: 0, r: 0 };
    let mut board: Board = Board::new();

    // Event loop.
    'running: loop {
        let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
        let mouse_pos = sdl2::rect::Point::new(mouse_state.x(), mouse_state.y());
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => direction = Direction::from((direction as i32) - 1),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => direction = Direction::from((direction as i32) + 1),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let previous_loc = location;
                    let try_loc = location + direction;
                    match board.get(try_loc) {
                        Some(x) => {
                            if !x.free {
                                break;
                            }
                            board.fill(try_loc);
                            board.free(previous_loc);
                            location = try_loc;
                        }
                        None => (),
                    }
                }
                _ => {}
            }
        }

        // Draw all tiles.
        board.draw(&mut canvas);
        match board.get(mouse_pos.into()) {
            Some(x) => {
                x.add_color(
                    &mut canvas,
                    mouse_pos.into(),
                    Color {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 70,
                    },
                );
            }
            None => (),
        }

        // Draw current position and direction.
        let arrow = direction.to_string();
        let mut text_center: sdl2::rect::Point = location.into();
        text_center.x -= 3;
        text_center.y -= 10;
        utils::render_text(&mut canvas, &font, &texture_creator, text_center, &arrow);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
