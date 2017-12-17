//! # My Game Engine.
//! Hopefully I will someday finish this.

extern crate cgmath;
extern crate genmesh;
#[macro_use]
extern crate gfx;
extern crate gfx_core;
pub extern crate gfx_device_gl;
extern crate image;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate specs;
extern crate shred;
extern crate toml;
extern crate winit;
extern crate glutin;
extern crate gfx_window_glutin;

pub use specs::{Component, VecStorage, World, System, DispatcherBuilder};

pub mod config;
pub mod game;
pub mod state;

pub mod window;
pub mod graphics;
pub mod screen;
pub mod systems;
