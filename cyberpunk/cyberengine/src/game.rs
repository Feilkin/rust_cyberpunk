//! Game Entry Point
use std::time::Instant;

use state::GameState;
use window;
use config;
use toml;

/// The default, single-window Game.
///
/// I was planning to make this a Trait, so users could implement their own Game objects,
/// but I think I like this approach more.
pub struct Game {
    config: config::GameConfig,
    window: window::Window,
}

impl Game {
    pub fn new() -> Game {
        let config = Self::load_config();
        let window = window::Builder::new()
            .with_title(config.graphics.title.clone())
            .with_dimensions(config.graphics.window_width, config.graphics.window_height)
            .with_vsync(config.graphics.vsync)
            .with_multisampling(config.graphics.multisampling)
            .build();
        Game {
            config: config,
            window: window,
        }
    }

    fn load_config() -> config::GameConfig {
        // TODO: I have no idea what I am doing
        toml::from_str(include_str!("../../src/game_config.toml")).unwrap()
    }

    pub fn play(mut self) -> () {
        // IDK poll the window or something

        let mut last_frame_update = Instant::now();

        let mut window = self.window;

        loop {
            let delta = last_frame_update.elapsed();

            if !window.poll_events() {
                break;
            }

            window.update(delta);

            last_frame_update = Instant::now();

            window = window.render();
        }
    }
}
