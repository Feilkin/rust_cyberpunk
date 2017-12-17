use std::mem;

use gfx;
use specs::{Component, System, ReadStorage, VecStorage, Join, RunNow};
use shred;

use graphics;

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

pub struct Sprite {
    pub texture: graphics::texture::Texture,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

pub struct TestSystem {
    pub renderer: Option<graphics::Renderer>,
}

impl<'a> System<'a> for TestSystem {
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

impl graphics::RenderingSystem for TestSystem {
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
