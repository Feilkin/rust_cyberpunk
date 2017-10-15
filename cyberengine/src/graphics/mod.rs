//! gfx-rs wrappers for ease of use

extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_core;

/// Type aliases for ease of use
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type Factory = gfx_device_gl::Factory;
pub type CBuffer = gfx_device_gl::CommandBuffer;
pub type Resources = gfx_device_gl::Resources;
pub type Encoder = gfx::Encoder<Resources, CBuffer>;
pub type RenderTargetView = gfx_core::handle::RenderTargetView<Resources, ColorFormat>;

pub trait Drawable {
    fn draw(&self, factory: &mut Factory, encoder: &mut Encoder, RenderTargetView) -> ();
}

pub mod texture;
