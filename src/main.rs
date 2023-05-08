mod direction;
mod hex;
mod utils;

use direction::Direction;
extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
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
    let mut direction: Direction = Direction::Right;
    let mut location: hex::Coordinates = hex::Coordinates { q: 0, r: 0 };
    let mut board: Vec<Vec<Option<hex::Tile>>> = vec![];
    for x in 0..15 {
        let row: Vec<Option<hex::Tile>> = vec![];
        board.push(row);
        for y in 0..15 {
            if x == 14 && y & 1 != 0 {
                board[x as usize].push(None);
                continue;
            }
            if (x == 0 || x == 14) && (y == 4 || y == 10) {
                board[x as usize].push(None);
                continue;
            }
            let mut free = !rng.gen_bool(0.08);
            if x == 0 && y == 0 {
                free = false;
            }
            let new_tile: hex::Tile = hex::Tile::new(hex::Coordinates::from_offset(x, y), free);
            board[x as usize].push(Some(new_tile));
        }
    }

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
                    let try_loc = location.add_direction(direction);
                    let previous_loc = location;
                    for row in board.iter_mut() {
                        for piece in row.iter_mut() {
                            if piece.is_some() {
                                if !piece.as_mut().unwrap().free {
                                    continue;
                                }
                                if piece.as_mut().unwrap().coordinates == try_loc {
                                    location = try_loc;
                                    piece.as_mut().unwrap().free = false;
                                }
                            }
                        }
                    }
                    if location == previous_loc {
                        break;
                    }
                    for row in board.iter_mut() {
                        for piece in row.iter_mut() {
                            if piece.is_some() {
                                if piece.as_mut().unwrap().coordinates == previous_loc {
                                    piece.as_mut().unwrap().free = true;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        for row in &board {
            for piece in row {
                if piece.is_some() {
                    piece.as_ref().unwrap().draw(&mut canvas);
                }
            }
        }

        let arrow = direction.to_string();
        utils::render_text(
            &mut canvas,
            &font,
            &texture_creator,
            location.into(),
            &arrow,
        );
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
