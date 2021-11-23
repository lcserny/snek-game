use std::io::stdout;
use crate::game::Game;

mod direction;
mod command;
mod point;
mod snake;
mod game;

pub fn run() {
    Game::new(stdout(), 10, 10).run();
}