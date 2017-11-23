//! Holds rendering context and stuff, IDK
use std::time::Duration;

use graphics;
use state;

pub struct Screen {
    renderer: graphics::Renderer,
    statemanager: state::Manager,
}

impl Screen {
    pub fn new(renderer: graphics::Renderer) -> Screen {
        Screen {
            renderer: renderer,
            statemanager: state::Manager::new(),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.statemanager.update(delta);
    }

    pub fn render(&mut self) -> () {
        self.renderer.clear();
        self.statemanager.render(&mut self.renderer);
        self.renderer.flush();
    }

    pub fn cleanup(&mut self) -> () {
        self.renderer.cleanup();
    }
}
