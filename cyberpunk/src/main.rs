extern crate cyberengine;
#[macro_use]
extern crate imgui;
#[macro_use]
extern crate gfx;
use gfx::traits::FactoryExt;

use cyberengine::{game, scene, resource};
use cyberengine::graphics::Drawable;
use imgui::*;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

#[derive(Debug)]
struct Game {
    map: cyberengine::resource::tilemap::Tilemap,
    is_running: bool,
}

impl game::Playable for Game {
    fn new() -> Game {
        Game {
            is_running: true,
            map: cyberengine::resource::tilemap::Tilemap::new(),
        }
    }

    fn load(&mut self, factory: &mut cyberengine::graphics::Factory) -> () {
        self.map =
            cyberengine::resource::tilemap::load_tiled("resources/maps/offices.json", factory)
                .expect("failed to load map");
    }

    #[allow(unused_variables)]
    fn tick<'a>(&mut self, ui: &'a Ui, dt: f32) -> () {
        ui.window(im_str!("Hello world"))
            .size((400.0, 600.0), ImGuiSetCond_Once)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("FPS: {:.0}", 1.0 / dt));
                ui.separator();
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));
                if ui.collapsing_header(im_str!("Tilemap Info")).build() {
                    let map: &mut resource::tilemap::Tilemap = &mut self.map;
                    ui.text(im_str!("filename           {}", map.filename));
                    ui.text(im_str!("JSON format:       {}", map.version));
                    ui.text(im_str!("width (in tiles)   {}", map.width));
                    ui.text(im_str!("height (in tiles)  {}", map.height));
                    ui.text(im_str!("width (in pixels)  {}", map.width * map.tilewidth));
                    ui.text(im_str!(
                        "height (in pixels) {}",
                        map.height * map.tileheight
                    ));
                    if ui.collapsing_header(im_str!("Layers")).build() {
                        for l in &mut map.layers {
                            ui.tree_node(im_str!("{}", l.name))
                                .label(im_str!("{}", l.name))
                                .build(|| {
                                    ui.text(im_str!("type              {}", l.layertype));
                                    ui.text(im_str!("width (in tiles)  {}", l.width));
                                    ui.text(im_str!("height (in tiles) {}", l.height));
                                    ui.checkbox(im_str!("visible?"), &mut l.visible);
                                })
                        }
                    }
                    if ui.collapsing_header(im_str!("Tilesets")).build() {
                        for ts in &mut map.tilesets {
                            ui.tree_node(im_str!("{}", ts.name)).build(|| {
                                ui.text(im_str!("root path {}", ts.root));
                                ui.text(im_str!(
                                    "image     <root>/{}",
                                    match ts.image {
                                        Some(ref img) => img,
                                        None => " NULL",
                                    }
                                ));
                                ui.text(im_str!("First GID {}", ts.firstgid));
                            });
                        }
                    }
                }
            });
    }

    fn render(
        &mut self,
        factory: &mut cyberengine::graphics::Factory,
        encoder: &mut cyberengine::graphics::Encoder,
        out: cyberengine::graphics::RenderTargetView,
    ) -> () {
        let ts = self.map.tilesets.get(0).unwrap();
        let tex = ts.get_texture().unwrap();
        tex.draw(factory, encoder, out);
    }

    fn running(&self) -> bool {
        self.is_running
    }
}

fn main() -> () {
    use cyberengine::game::Playable;
    let mut game: Game = Game::new();
    game.run("neekeri".to_owned(), [0.0, 0.0, 0.0, 1.0]);
}
