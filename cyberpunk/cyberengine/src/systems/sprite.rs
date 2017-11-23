use gfx;
use specs::{Component, System, ReadStorage, VecStorage, Join};

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

pub struct TestSystem<'a> {
    pub renderer: Option<&'a mut graphics::Renderer>,
}

impl<'a> System<'a> for TestSystem<'a> {
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
