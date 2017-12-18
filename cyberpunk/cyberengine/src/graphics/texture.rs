//! High-level wrapper for gfx-rs Textures and related stuff
use std::cmp;
use cgmath::{Matrix4, SquareMatrix, Vector4};
use gfx;
use gfx::traits::FactoryExt;
//use genmesh::{Quad, EmitTriangles, MapVertex, Triangulate, MapToVertices, Vertices};
//use genmesh::generators::Plane;
use image;

use graphics;
use resource;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2]  = "a_Uv",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        projection_cb: gfx::ConstantBuffer<graphics::ModelViewProjection> = "b_VsLocals",
        out: gfx::BlendTarget<graphics::ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

pub struct Builder {
    dimensions: Option<(u32, u32)>,
    indices: Option<Vec<u32>>,
    vertex_data: Option<Vec<Vertex>>,
    view: Option<graphics::ShaderResourceView>,
    filename: Option<String>,
    flip_y: bool,
}


impl Builder {
    pub fn new() -> Builder {
        Builder {
            dimensions: None,
            indices: None,
            vertex_data: None,
            view: None,
            filename: None,
            flip_y: false,
        }
    }

    fn default_vertex_data_and_index(d: (u32, u32)) -> (Vec<Vertex>, Vec<u32>) {
        let (hw, hh) = (d.0 as f32 / 2., d.1 as f32 / 2.);
        println!("vittu: {:?}", (hw, hh));
        (vec![
            Vertex { pos: [-hw, -hh], uv: [0., 1.], color: [1., 1., 1., 1.]},
            Vertex { pos: [hw, -hh], uv: [1., 1.], color: [1., 1., 1., 1.]},
            Vertex { pos: [hw, hh], uv: [1., 0.], color: [1., 1., 1., 1.]},
            Vertex { pos: [-hw, hh], uv: [0., 0.], color: [1., 1., 1., 1.]},
        ],
        vec![0, 1, 2, 2, 3, 0])
    }

    pub fn with_flipped_y(mut self) -> Builder {
        self.flip_y = true;
        self
    } 

    pub fn from_file(mut self, filename: String) -> Builder {
        self.filename = Some(filename);
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Builder {
        self.dimensions = Some((width, height));
        self
    }

    pub fn with_view(mut self, view: graphics::ShaderResourceView) -> Builder {
        self.view = Some(view);
        self
    }

    pub fn with_vertex_data_and_indices(mut self, vertex_data: Vec<Vertex>, indices: Vec<u32>) -> Builder {
        self.vertex_data = Some(vertex_data);
        self.indices = Some(indices);
        self
    }

    pub fn build(self, factory: &mut graphics::Factory) -> Texture {
        let mut dimensions = match self.dimensions {
            Some(dimensions) => dimensions,
            None => (0, 0),
        };

        let view = match self.view {
            Some(view) => view,
            None => {
                match self.filename {
                    Some(filename) => {
                        use gfx::texture as t;
                        use gfx_core::Factory;
                        let img = image::open(filename).unwrap().to_rgba();
                        let (width, height) = img.dimensions();
                        let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
                        let (_, view) = factory
                            .create_texture_immutable_u8::<graphics::ColorFormat>(kind, &[&img])
                            .unwrap();

                        dimensions = (width, height);
                        view
                    },
                    None => {
                        panic!("TextureBuilder needs to have either view or filename!");
                    }
                }
            },
        };

        let (mut vertex_data, indices) = match self.vertex_data {
            Some(vertex_data) => (vertex_data, self.indices.unwrap()),
            None => Builder::default_vertex_data_and_index(dimensions),
        };

        if self.flip_y {
            vertex_data = vertex_data.iter().map(|v| Vertex {
                pos: v.pos,
                uv: [v.uv[0], 1. - v.uv[1]],
                color: v.color
            }).collect();
        }

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &indices as &[u32]);

        Texture {
            dimensions: dimensions,
            slice: slice,
            view: view,
            vbuf: vbuf,
        }
    }
}

#[derive(Clone)]
pub struct Texture {
    dimensions: (u32, u32),
    view: graphics::ShaderResourceView,
    pub vbuf: gfx::handle::Buffer<graphics::Resources, Vertex>,
    pub slice: gfx::Slice<graphics::Resources>,
}

impl Texture {
    pub fn clone_view(&self) -> graphics::ShaderResourceView {
        self.view.clone()
    }
}

