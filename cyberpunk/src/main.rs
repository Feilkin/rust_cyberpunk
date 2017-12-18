//! Game implementation with Cyberengine

#![allow(unused_variables)]

extern crate cyberengine;
#[macro_use]
extern crate imgui;

use cyberengine::game::Game;

fn main() -> () {
    let mut game = Game::new();
    game.play();
}
