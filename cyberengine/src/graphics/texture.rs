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

#[derive(Debug)]
pub struct TextureBuilder<M, V> {
    pso: graphics::PipelineState<M>,
    view: Option<graphics::ShaderResourceView>,
    filename: Option<String>,
    vbuf: Option<gfx_core::handle::Buffer<graphics::Resources, V>>,
    slice: Option<gfx::Slice<graphics::Resources>>,
    width: Option<usize>,
    height: Option<usize>,
}

impl<M, V> TextureBuilder<M, V> {
    pub fn new(pso: graphics::PipelineState<M>) -> TextureBuilder<M, V> {
        TextureBuilder {
            pso: pso,
            view: None,
            vbuf: None,
            slice: None,
            indices: None,
            width: None,
            height: None,
        }
    }

    pub fn with_view<M, V>(self, view: graphics::ShaderResourceView, width: usize, height: usize) -> TextureBuilder<M, V> {
        match self.filename {
            Some(_) => panic!("Tried to create a Texture from view, but already had filename set {:?}", self),
            None => (),
        };
        self.view = Some(view);
        self
    }

    pub fn from_file<M, V>(self, filename: String) -> TextureBuilder<M, V> {
        match self.view {
            Some(_) => panic!("Tried  to create a Texture from file, but already had view set {:?}", self);,
            None => (),
        };
        self.filename = Some(filename);
        self
    }

    pub fn with_vbuf<M, V>(self, vbuf: gfx_core::handle::Buffer<graphics::Resources, V>) -> TextureBuilder<M, V> {
        self.vbuf = Some(vbuf);
        self
    }

    pub fn with_slice<M, V>(self, slice: gfx::Slice<graphics::Resources>) -> TextureBuilder<M, V> {
        self.slice = Some(slice);
        self
    }

    pub fn build<M, V>(self, factory: graphics::Factory) -> Result<Texture<M, V>, String> {
        let (view, width, height) = match self.view {
            Some(view) => (view, self.width.unwrap(), self.height.unwrap()),
            None => {
                match self.filename {
                    Some(filename) => {
                        use gfx::texture as t;
                        let img = image::open(self.filename).unwrap().to_rgba();
                        let (width, heigh) = img.dimensions();
                        let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
                        let (_, view) = factory
                            .create_texture_immutable_u8::<graphics::ColorFormat>(kind, &[&img])
                            .unwrap();
                        (view, width, height)
                    },
                    None => Err("TextureBuilder needs to have either view or filename set!"),
                }
            },
        }

        let (vbuf, slice) = match self.vbuf {
            Some(vbuf) => (vbuf, self.slice.unwrap()),
            None => {
                let vertex_data: Vec<V> = Plane::new()
                    .vertex(|v| {
                        V::new(v.pos[0], v.pos[1])
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
                                uv: [0., 0.],
                                color: v0.color,
                            },
                            Vertex {
                                pos: v1.pos,
                                uv: [1., 0.],
                                color: v1.color,
                            },
                            Vertex {
                                pos: v2.pos,
                                uv: [1., 1.],
                                color: v2.color,
                            },
                            Vertex {
                                pos: v3.pos,
                                uv: [0., 1.],
                                color: v3.color,
                            },
                        )
                    })
                    .triangulate()
                    .vertices()
                    .collect();

                let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    view: graphics::ShaderResourceView,
    pso: gfx::pso::PipelineState<graphics::Resources, pipe_tex::Meta>,
    vbuf: gfx_core::handle::Buffer<graphics::Resources, Vertex>,
    slice: gfx::Slice<graphics::Resources>,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn from_view(factory: &mut graphics::Factory, view: graphics::ShaderResourceView) -> Texture{
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
                        uv: [0., 0.],
                        color: v0.color,
                    },
                    Vertex {
                        pos: v1.pos,
                        uv: [1., 0.],
                        color: v1.color,
                    },
                    Vertex {
                        pos: v2.pos,
                        uv: [1., 1.],
                        color: v2.color,
                    },
                    Vertex {
                        pos: v3.pos,
                        uv: [0., 1.],
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

    pub fn from_file(factory: &mut graphics::Factory, filename: &str) -> Texture {
        let view = load_texture(factory, filename).unwrap();

        let vertex_data: Vec<Vertex> = Plane::subdivide(32, 32)
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
                        uv: [0., 64. / 1280.],
                        color: v0.color,
                    },
                    Vertex {
                        pos: v1.pos,
                        uv: [64. / 1728., 64. / 1280.],
                        color: v1.color,
                    },
                    Vertex {
                        pos: v2.pos,
                        uv: [64. / 1728., 0.],
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
    use gfx::texture as t;
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<graphics::ColorFormat>(kind, &[&img])
        .unwrap();
    Ok(view)
}
