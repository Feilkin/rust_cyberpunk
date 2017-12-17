//! gfx-rs wrappers for ease of use

use cgmath;
use cgmath::{Deg, Matrix4, SquareMatrix, Vector3, Vector4, Point3};

use gfx;
use gfx_core;

use specs;
use shred;

const CLEAR_COLOR: [f32; 4] = [0., 0., 0., 1.];

pub use gfx_device_gl as backend;

// Type aliases for ease of use
pub type Device = backend::Device;
pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type Factory = backend::Factory;
pub type Resources = backend::Resources;
pub type Encoder = gfx::Encoder<Resources, backend::CommandBuffer>;
pub type RenderTargetView = gfx_core::handle::RenderTargetView<Resources, ColorFormat>;
pub type DepthStencilView = gfx_core::handle::DepthStencilView<Resources, DepthFormat>;
pub type ShaderResourceView = gfx::handle::ShaderResourceView<Resources, [f32; 4]>;
pub type PipelineState<M> = gfx::pso::PipelineState<Resources, M>;

gfx_constant_struct! {
    ModelViewProjection {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }
}

pub mod texture;

pub struct Renderer {
    pub factory: Factory,
    device: Device,
    main_target: RenderTargetView,
    main_depth: DepthStencilView,
    pub encoder: Encoder,

    pub debug_texture: texture::Texture,

    // PSO's
    pso_texture: PipelineState<texture::pipe::Meta>,
    linear_sampler: gfx_core::handle::Sampler<Resources>,
}

impl Renderer {
    pub fn new(
        mut factory: Factory,
        device: Device,
        main_target: RenderTargetView,
        main_depth: DepthStencilView,
    ) -> Renderer {
        use gfx::traits::FactoryExt;
        let encoder = factory.create_command_buffer().into();
        let sampler = factory.create_sampler_linear();

        let pso_texture = factory
            .create_pipeline_simple(
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/texture_400.glslv"
                )),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/shaders/texture_400.glslf"
                )),
                texture::pipe::new(),
            )
            .unwrap();

        let debug_texture = texture::Builder::new()
            .from_file("test.jpg".to_owned())
            .build(&mut factory);

        Renderer {
            factory: factory,
            device: device,
            encoder: encoder,
            main_target: main_target,
            main_depth: main_depth,

            debug_texture: debug_texture,

            pso_texture: pso_texture,
            linear_sampler: sampler,
        }
    }

    pub fn clear(&mut self) -> () {
        self.encoder.clear(&self.main_target, CLEAR_COLOR);
    }

    pub fn flush(&mut self) -> () {
        self.encoder.flush(&mut self.device);
    }

    pub fn cleanup(&mut self) -> () {
        use gfx_core::Device;
        self.device.cleanup();
    }

    /// Draws a texture to the main target of the renderer.
    pub fn draw_texture(&mut self, texture: &texture::Texture, position: (f32, f32)) -> () {
        use gfx::traits::FactoryExt;

        let data = texture::pipe::Data {
            texture: (texture.clone_view(), self.linear_sampler.clone()),
            vbuf: texture.vbuf.clone(),
            out: self.main_target.clone(),
            projection_cb: self.factory.create_constant_buffer(1),
        };

        let mvp = ModelViewProjection {
            model: Matrix4::from_translation(Vector3::new(position.0, position.1, 0.)).into(),
            view: Matrix4::look_at(
                Point3::new(0., 0., 720.),
                Point3::new(0., 0., 0.),
                Vector3::unit_y(),
            ).into(),
            proj: cgmath::perspective(Deg(60.0f32), 1280. / 720., 0.1, 5000.).into(),
        };
        self.encoder.update_constant_buffer(
            &data.projection_cb,
            &mvp,
        );

        self.encoder.draw(&texture.slice, &self.pso_texture, &data);
    }
}


pub trait RenderingSystem {
    /// Render the world with the system, and return the Renderer (unharmed)
    fn render_world<'s, 'r>(
        &'s mut self,
        res: &'r mut shred::Resources,
        renderer: Renderer,
    ) -> Renderer;
}
