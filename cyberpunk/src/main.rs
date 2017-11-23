//! Game implementation with Cyberengine

extern crate cyberengine;
#[macro_use]
extern crate imgui;

use cyberengine::game::Game;

fn main() -> () {
    let mut game = Game::new();

    game.play();
}
