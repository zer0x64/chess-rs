use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use graphics::*;

const WINDOW_WIDTH: u32 = 400;
const WINDOW_HEIGHT: u32 = 400;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(Clone)]
struct Piece {
    piece_type: String,
    player: bool,
}

struct Board {
    pieces: Vec<Option<Piece>>
}

impl Board {
    fn new() -> Board {
        let pieces = vec![None; 8 * 8];

        Board {
            pieces
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new(
        "chess-rs",
        [WINDOW_WIDTH, WINDOW_HEIGHT]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let mut board = Board::new();

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            let square = rectangle::rectangle_by_corners(0.0, 0.0, 1.0, 1.0);

            gl.draw(r.viewport(), |c, gl| {
                clear( WHITE, gl);

                // Draw board
                for i in 0..board.pieces.len() {
                    let transformation =
                        c.transform.trans(
                            (i % 8) as f64 * (r.width / 8.0),
                            (i /  8) as f64 * (r.height / 8.0))
                            .scale(r.width / 8.0, r.height / 8.0);
                    let color = if (i / 8 + i % 8) % 2 == 0 { BLACK } else { WHITE };

                    rectangle(color, square, transformation, gl);
                }
            });
        }
    }

    println!("Hello, world!");
}
