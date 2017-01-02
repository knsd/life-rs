/// The Gmae of Life is my first experimental Rust project
/// to learn base features of the language.

extern crate piston_window;

mod symvec;
mod board;
mod engine;

use piston_window::*;
use engine::Engine;
use board::Board;
use std::cell::RefCell;
use std::rc::Rc;

enum State {
    Working,
    Paused,
    Help,
}

struct Game {
    width: u32,
    height: u32,

    window: Rc<RefCell<PistonWindow>>,
    engine: Engine,
    cur_state: State,
}

impl Game {

    fn new(width: u32, height: u32) -> Game {

        let mut window: PistonWindow = WindowSettings::new(
            "My Rust Life",
            [width, height]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut game_board = Board::new(5, 5);

        game_board.born_at(0, 0);
        game_board.born_at(1, 1);

        Game {width: width, height: height,
              window: Rc::new(RefCell::new(window)),
              engine: Engine::new(game_board),
              cur_state: State::Paused}
    }

    fn event_dispatcher(&self) {
        while true {

            let event = { self.window.borrow_mut().next() };

            match event {

                Some(e) => {
                    if let Event::Render(_) = e {
                        self.paint(&e);
                    }
                }
                None => break

            }
        }
    }

    fn to_screen(&self, col: isize, row: isize) {

    }

    fn paint(&self, e: &Event) {

        self.window.borrow_mut().draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            let board = self.engine.get_board();

            for (coords, is_alive) in board.into_iter() {
                let col = coords.col;
                let row = coords.row;

                self.to_screen(col, row);
                rectangle([0.5, 1.0, 0.0, 0.3],
                          [0.0, 0.0, 10.0, 10.0],
                          c.transform, g);
            }

        });

    }

}

fn main() {

    let mut game = Game::new(1024, 768);
    game.event_dispatcher();

}
