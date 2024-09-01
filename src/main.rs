/// Utility functions to play on a gameboard comprised of hexagonal tiles, or "hexes".
mod board;
mod creature;
/// Damage computing logic.
mod damage;
/// All the [Dice][crate::dice::Dice] throwing logic.
mod dice;
/// UTF-8 + SDL2 shenanigans.
mod utils;

use board::board::Board;
use board::coordinates::Coordinates;
use board::direction::Direction;
use std::path::PathBuf;

extern crate sdl2;

use crate::board::coordinates::FloatCoordinates;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

fn main() {
    // SDL init.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("DRG: The Boardgame", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not build window");

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().unwrap();
    let manager = sdl2::ttf::init().unwrap();
    let _loader = sdl2::image::init(sdl2::image::InitFlag::PNG).expect("could not init SDL2_image");
    let texture_creator = canvas.texture_creator();
    let texture = utils::TextureAtlas::new_from_path(
        &texture_creator,
        PathBuf::from("/home/cleo/Documents/dev/rust/drg/test_asset.png"),
    )
    .expect("could not load texture");
    let font = utils::load_fonts(&manager, "Unifont").expect("could not load font");

    canvas.present();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not start event pump");

    // Map generation.
    let mut direction: Direction = Direction::Right;
    let mut location: Coordinates = Coordinates { q: 0, r: 0 };
    let mut board: Board = Board::new();

    // Debug options.
    let mut display_pos: bool = false;
    let mut display_path: bool = false;
    let mut line_up: bool = false;
    let mut display_los: bool = false;

    // Event loop.
    'running: loop {
        let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
        let mouse_pos = Point::new(mouse_state.x(), mouse_state.y());
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    let coords = Coordinates::from(Point::new(x, y));
                    let tile = board.get(coords);
                    if tile.is_none() {
                        continue;
                    }
                    let tile = tile.unwrap();
                    if tile.free {
                        board.fill(coords);
                    } else {
                        board.free(coords);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => direction = Direction::from((direction as i32) - 1),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => direction = Direction::from((direction as i32) + 1),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    display_pos = true;
                    display_path = false;
                    display_los = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => display_pos = false,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    display_pos = false;
                    display_path = false;
                    display_los = !display_los;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    display_path = true;
                    display_pos = false;
                    display_los = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::P),
                    ..
                } => display_pos = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => line_up = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => line_up = false,
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

        // Draw tile on mouse;
        if let Some(_tile) = board.get(mouse_pos.into()) {
            let line = location.line(mouse_pos.into());
            let chosen_line: Vec<Coordinates>;
            if display_path {
                chosen_line = board.path(location, mouse_pos.into()).unwrap_or(vec![]);
            } else if line_up {
                chosen_line = line.0;
            } else {
                chosen_line = line.1;
            }
            let coords: Coordinates = mouse_pos.into();
            let pos: Point = coords.into();
            /*
            texture
                .render(&mut canvas, pos.offset(-30, -30))
                .expect("could not render texture");
             */
            /*
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
            */
            if display_pos || display_path {
                utils::render_text(
                    &mut canvas,
                    &font,
                    &texture_creator,
                    Point::new(900, 1000),
                    location.distance(mouse_pos.into()).to_string().as_str(),
                );
                chosen_line.iter().for_each(|coord| {
                    board.get(*coord).unwrap().add_color(
                        &mut canvas,
                        *coord,
                        Color {
                            r: 0,
                            g: 150,
                            b: 0,
                            a: 70,
                        },
                    )
                });
            }
        }

        if display_los {
            let tiles = board.tiles.iter().map(|(coord, tile)| {
                if !tile.free {
                    return (coord, false);
                }

                let res = location.strict_line(&board, *coord, |option| {
                    if option.is_none() {
                        return false;
                    }
                    option.unwrap().free
                });

                if res {
                    return (coord, true);
                }
                (coord, false)
            });
            tiles.for_each(|(coords, visible)| {
                let tile = board.get(*coords);
                if tile.is_none() {
                    return;
                }
                let tile = tile.unwrap();
                if !tile.free {
                    return;
                }
                let color: Color;
                if visible {
                    color = Color::RGB(125, 255, 125);
                } else {
                    color = Color::RGB(255, 125, 125);
                }
                tile.add_color(&mut canvas, *coords, color)
            })
        }
        // Draw current position and direction.
        let arrow = direction.to_string();
        let mut text_center: Point = location.into();
        text_center.x -= 3;
        text_center.y -= 10;
        utils::render_text(&mut canvas, &font, &texture_creator, text_center, &arrow);
        if display_los {
            utils::render_text(
                &mut canvas,
                &font,
                &texture_creator,
                Point::new(1200, 475),
                "mode (F): display line of sight (simple)",
            );
        } else if display_path {
            utils::render_text(
                &mut canvas,
                &font,
                &texture_creator,
                Point::new(1200, 475),
                "mode (P): path",
            );
        } else if display_pos {
            utils::render_text(
                &mut canvas,
                &font,
                &texture_creator,
                Point::new(1200, 475),
                "mode (D): line",
            );
        } else {
            utils::render_text(
                &mut canvas,
                &font,
                &texture_creator,
                Point::new(1200, 475),
                "mode: default",
            );
        }
        utils::render_text(
            &mut canvas,
            &font,
            &texture_creator,
            Point::new(1200, 500),
            FloatCoordinates::from(mouse_pos).to_string().as_str(),
        );
        utils::render_text(
            &mut canvas,
            &font,
            &texture_creator,
            Point::new(1200, 525),
            Coordinates::from(mouse_pos).to_string().as_str(),
        );
        utils::render_text(
            &mut canvas,
            &font,
            &texture_creator,
            Point::new(1200, 550),
            &format!("Player position: {}", location),
        );

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
