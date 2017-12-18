use std::mem;
use std::collections::HashMap;

use gfx;
use specs::{Component, System, ReadStorage, WriteStorage, Entities, VecStorage, Join, RunNow,
            ParJoin, Fetch, LazyUpdate, Entity};
use shred;
use rayon::iter::ParallelIterator;

use graphics;
use graphics::texture;
use resource;

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct SpriteSpawn {
    pub texture_identifier: resource::Identifier,
}

impl Component for SpriteSpawn {
    type Storage = VecStorage<Self>;
}

pub struct Sprite {
    pub texture: graphics::texture::Texture,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

pub struct SpriteLoader {
    texture_cache: HashMap<String, graphics::texture::Texture>,
    pub renderer: Option<graphics::Renderer>,
}

impl SpriteLoader {
    pub fn new() -> SpriteLoader {
        SpriteLoader {
            renderer: None,
            texture_cache: HashMap::new(),
        }
    }
}

impl<'a> System<'a> for SpriteLoader {
    type SystemData = (WriteStorage<'a, SpriteSpawn>, WriteStorage<'a, Sprite>, Entities<'a>);

    fn run(&mut self, (mut spawns, mut sprites, entities): Self::SystemData) {
        let mut to_remove: Vec<Entity> = Vec::new();
        match self.renderer {
            None => panic!("No renderer"),
            Some(ref mut renderer) => {
                for (entity, spawn) in (&*entities, &spawns).join() {
                    use resource::Identifier;

                    let texture = match spawn.texture_identifier {
                        Identifier::Image(ref filename) => {
                            if self.texture_cache.contains_key(filename) {
                                self.texture_cache.get(filename).unwrap().clone()
                            } else {
                                println!("Loading {:?}!", filename);
                                let texture = texture::Builder::new()
                                    .from_file(filename.to_owned())
                                    .build(&mut renderer.factory);
                                self.texture_cache.insert(
                                    filename.to_owned(),
                                    texture.clone(),
                                );
                                texture
                            }
                        }
                    };

                    sprites.insert(entity, Sprite { texture: texture });
                    to_remove.push(entity);
                }
            }
        }

        for e in &to_remove {
            spawns.remove(*e);
        }

    }
}

pub struct SpriteRenderer {
    pub renderer: Option<graphics::Renderer>,
}

impl<'a> System<'a> for SpriteRenderer {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

    fn run(&mut self, (position, sprite): Self::SystemData) {
        match self.renderer {
            None => panic!("No renderer"),
            Some(ref mut renderer) => {
                for (position, sprite) in (&position, &sprite).join() {
                    renderer.draw_texture(&sprite.texture, (position.x, position.y));
                }
            }
        }
    }
}

impl graphics::RenderingSystem for SpriteRenderer {
    fn render_world<'s, 'r>(
        &'s mut self,
        res: &'r mut shred::Resources,
        renderer: graphics::Renderer,
    ) -> graphics::Renderer {
        use specs::RunNow;

        {
            self.renderer = Some(renderer);
            self.run_now(res);
        }
        let renderer = mem::replace(&mut self.renderer, None);

        match renderer {
            Some(renderer) => renderer,
            None => {
                panic!("No renderer after render??");
            }
        }
    }
}

impl graphics::RenderingSystem for SpriteLoader {
    fn render_world<'s, 'r>(
        &'s mut self,
        res: &'r mut shred::Resources,
        renderer: graphics::Renderer,
    ) -> graphics::Renderer {
        use specs::RunNow;

        {
            self.renderer = Some(renderer);
            self.run_now(res);
        }
        let renderer = mem::replace(&mut self.renderer, None);

        match renderer {
            Some(renderer) => renderer,
            None => {
                panic!("No renderer after render??");
            }
        }
    }
}
