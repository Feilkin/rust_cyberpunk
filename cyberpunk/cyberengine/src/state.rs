//! Traits for GameStates.
//!
//! Single file for now, will expand on if necessary

// TODO: Figure out what is the best way for defining states.

use std::collections::HashMap;
use std::time::Duration;

use rand;
use specs;

use game::Game;
use graphics;
use systems::DeltaTime;
use systems::sprite::{SpriteRenderer, Position, Sprite, SpriteSpawn, SpriteLoader};
use resource;

pub struct GameState {
    name: &'static str,
    loaded: bool,
    pub world: specs::World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    rendering_systems: Vec<Box<graphics::RenderingSystem>>,
}

impl GameState {
    pub fn new<'s, F>(name: &'static str, world_init: F, rendering_systems: Vec<Box<graphics::RenderingSystem>>)
    ->
    GameState
where
    F: Fn(&mut specs::World) -> specs::Dispatcher<'static, 'static> {
        let mut world = specs::World::new();
        world.add_resource(DeltaTime(Duration::new(0, 0)));
        let dispatcher = world_init(&mut world);

        GameState {
            name: name,
            loaded: false,
            world: world,
            dispatcher: dispatcher,
            rendering_systems: rendering_systems,
        }
    }

    /// Preloads necessary resources for showing this State.
    /// Note that expensive loading should be done in loading screens (which I hopefully implement later)
    fn preload(&mut self) -> () {
    }

    /// Initializes the State, called every time the state is set active.
    fn enter(&mut self) -> () {
    }

    /// Updates the game state.
    fn update(&mut self, dt: Duration) -> () {
        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt);
        }
        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();
    }

    /// Renders the scene.
    fn render(&mut self, mut renderer: graphics::Renderer) -> graphics::Renderer {
        for rs in self.rendering_systems.iter_mut(){
            let res = &mut self.world.res;
            renderer = rs.render_world(res, renderer);
        }
        renderer
    }

    /// Called when the State is left.
    fn leave(&mut self) -> () {}
}

struct SpriteSpawner {}

impl<'r> specs::System<'r> for SpriteSpawner {
    type SystemData = (specs::ReadStorage<'r, Sprite>,
        specs::WriteStorage<'r, SpriteSpawn>,
        specs::WriteStorage<'r, SpriteDuration>,
        specs::WriteStorage<'r, Position>,
        specs::Entities<'r>);

    fn run(&mut self, (sprites, mut spawns, mut durs, mut pos, entities): Self::SystemData) {
        use specs::Join;

        let c = (&sprites).join().count();
        if c < 100 {
            let e = entities.create();
            pos.insert(e, Position { x: rand::random::<f32>() * 1000. - 500., y: rand::random::<f32>() * 1000. - 500.});
            spawns.insert(e, SpriteSpawn { texture_identifier: resource::Identifier::Image("test.jpg".to_owned()) });
            durs.insert(e, SpriteDuration(Duration::new(0,0)));
        }
    }
}

struct SpriteDespawner {}
struct SpriteDuration(Duration);

impl specs::Component for SpriteDuration {
    type Storage = specs::VecStorage<Self>;
}


impl<'r> specs::System<'r> for SpriteDespawner {
    type SystemData = (specs::WriteStorage<'r, SpriteDuration>,
        specs::Entities<'r>,
        specs::Fetch<'r, DeltaTime>);

    fn run(&mut self, (mut sprites, entities, dt): Self::SystemData) {
        use specs::Join;
        let timeout = Duration::from_millis(500);

        for (sprite, entity) in (&mut sprites, &*entities).join() {
            let &mut SpriteDuration(ref mut dur) = sprite;
            *dur = *dur + dt.0;

            if *dur > timeout {
                entities.delete(entity).unwrap();
            }


        }
    }
}

fn splash_state() -> GameState{
    GameState::new("splash",
        |world| {
            world.register::<Position>();
            world.register::<Sprite>();
            world.register::<SpriteSpawn>();
            world.register::<SpriteDuration>();

            let dispatcher: specs::Dispatcher = specs::DispatcherBuilder::new()
                .add(SpriteSpawner {}, "sprite_spawner", &[])
                .add(SpriteDespawner {}, "sprite_despawner", &[])
                .build();

            dispatcher
        },
        vec![
            Box::new(SpriteLoader::new()),
            Box::new(SpriteRenderer { renderer: None })
        ])
}

pub struct Manager {
    states: HashMap<&'static str, GameState>,
    pub current_state: &'static str,
    next_state: Option<&'static str>,
}

impl Manager {
    pub fn new() -> Manager {
        let mut states = HashMap::new();
        let mut splash_state = splash_state();
        splash_state.preload();
        states.insert("splash", splash_state);

        Manager {
            states: states,
            current_state: "splash",
            next_state: None,
        }
    }

    pub fn add_state(&mut self, state: GameState) -> () {
        self.states.insert(state.name, state);
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

    pub fn render(&mut self, mut renderer: graphics::Renderer) -> graphics::Renderer {
        let current_state = self.states.get_mut(self.current_state).unwrap();
        renderer = current_state.render(renderer);
        renderer
    }
}