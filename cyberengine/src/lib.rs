#[allow(unused_imports)]
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[allow(unused_imports)]
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate genmesh;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub mod game;
pub mod resource;
pub mod scene;
pub mod graphics;
