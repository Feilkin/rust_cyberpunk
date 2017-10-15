//! Wrapper for textures for ease of use
use gfx;
use gfx_core;
use gfx::Factory;
use gfx::traits::FactoryExt;
use image;
use graphics;
use genmesh::{Quad, EmitTriangles, MapVertex, Triangulate, MapToVertices, Vertices};
use genmesh::generators::Plane;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2]  = "a_Uv",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe_tex {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        out: gfx::BlendTarget<graphics::ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    view: gfx::handle::ShaderResourceView<graphics::Resources, [f32; 4]>,
    slice: gfx::Slice<graphics::Resources>,
    pso: gfx::pso::PipelineState<graphics::Resources, pipe_tex::Meta>,
    vbuf: gfx_core::handle::Buffer<graphics::Resources, Vertex>,
}

impl Texture {
    pub fn from_file(factory: &mut graphics::Factory, filename: &str) -> Texture {
        let view = load_texture(factory, filename).unwrap();

        let vertex_data: Vec<Vertex> = Plane::new()
            .vertex(|v| {
                Vertex {
                    pos: [v.pos[0], v.pos[1]],
                    uv: [0., 0.],
                    color: [1., 0., 1.],
                }
            })
            .map(|Quad {
                 x: v0,
                 y: v1,
                 z: v2,
                 w: v3,
             }| {
                Quad::new(
                    Vertex {
                        pos: v0.pos,
                        uv: [0., 1.],
                        color: v0.color,
                    },
                    Vertex {
                        pos: v1.pos,
                        uv: [1., 1.],
                        color: v1.color,
                    },
                    Vertex {
                        pos: v2.pos,
                        uv: [1., 0.],
                        color: v2.color,
                    },
                    Vertex {
                        pos: v3.pos,
                        uv: [0., 0.],
                        color: v3.color,
                    },
                )
            })
            .triangulate()
            .vertices()
            .collect();

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());

        let pso = factory
            .create_pipeline_simple(
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/texture_400.glslv"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/texture_400.glslf"
                )),
                pipe_tex::new(),
            )
            .unwrap();

        Texture {
            view: view,
            pso: pso,
            slice: slice,
            vbuf: vbuf,
        }
    }
}

impl graphics::Drawable for Texture {
    fn draw(
        &self,
        factory: &mut graphics::Factory,
        encoder: &mut graphics::Encoder,
        out: graphics::RenderTargetView,
    ) -> () {

        let sampler = factory.create_sampler_linear();
        let data = pipe_tex::Data {
            vbuf: self.vbuf.clone(),
            texture: (self.view.clone(), sampler),
            out: out,
        };
        encoder.draw(&self.slice, &self.pso, &data);
    }
}

// texture loading boilerplate
fn load_texture(
    factory: &mut graphics::Factory,
    path: &str,
) -> Result<gfx::handle::ShaderResourceView<graphics::Resources, [f32; 4]>, String> {
    use gfx::format::Rgba8;
    use gfx::texture as t;
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<Rgba8>(kind, &[&img])
        .unwrap();
    Ok(view)
}
