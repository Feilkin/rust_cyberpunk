//! Traits for GameStates.
//!
//! Single file for now, will expand on if necessary

use std::collections::HashMap;
use std::time::Duration;

use rand;
use specs;

use game::Game;
use graphics;
use systems::sprite::{TestSystem, Position, Sprite};

pub struct GameState {
    loaded: bool,
    pub world: specs::World,
}

impl GameState {
    pub fn new() -> GameState {
        let mut world = specs::World::new();
        world.register::<Position>();
        world.register::<Sprite>();

        GameState {
            loaded: false,
            world: world,
        }
    }

    /// Preloads necessary resources for showing this State.
    /// Note that expensive loading should be done in loading screens (which I hopefully implement later)
    fn preload(&mut self) -> () {}

    /// Initializes the State, called every time the state is set active.
    fn enter(&mut self) -> () {

    }

    /// Updates the game state.
    fn update(&mut self, delta: Duration) -> () {}

    /// Renders the scene.
    fn render(&mut self, renderer: &mut graphics::Renderer) -> () {
        use specs::RunNow;

        self.world.create_entity()
            .with(Position { x: rand::random::<f32>() * 1000. - 500., y: rand::random::<f32>() * 1000. - 500.})
            .with(Sprite { texture: renderer.debug_texture.clone()});

        let mut renderer_system = TestSystem { renderer: Some(renderer) };
        renderer_system.run_now(&self.world.res);
    }

    /// Called when the State is left.
    fn leave(&mut self) -> () {}
}

fn splash_state() -> GameState {
    GameState::new()
}

pub struct Manager {
    states: HashMap<&'static str, GameState>,
    pub current_state: &'static str,
    next_state: Option<&'static str>,
}

impl Manager {
    pub fn new() -> Manager {
        let mut states = HashMap::new();
        states.insert("splash", splash_state());

        Manager {
            states: states,
            current_state: "splash",
            next_state: None,
        }
    }

    pub fn register_state(&mut self, name: &'static str, state: GameState) -> () {
        self.states.insert(name, state);
    }

    pub fn switch_state(&mut self, name: &'static str) -> () {
        assert!(self.states.contains_key(name));
        self.next_state = Some(name);
    }

    pub fn update(&mut self, delta: Duration) -> () {
        {
            let current_state = self.states.get_mut(self.current_state).unwrap();
            current_state.update(delta);
        }

        match self.next_state {
            Some(state_name) => {
                {
                    let last_state = self.states.get_mut(self.current_state).unwrap();
                    last_state.leave();
                }
                {
                    let next_state = self.states.get_mut(state_name).unwrap();
                    if !next_state.loaded { next_state.preload() }
                    next_state.enter();
                }
                self.current_state = state_name;
            },
            None => {},
        }
    }

    pub fn render(&mut self, renderer: &mut graphics::Renderer) -> () {
        let current_state = self.states.get_mut(self.current_state).unwrap();
        current_state.render(renderer);
    }
}