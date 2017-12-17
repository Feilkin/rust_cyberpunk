//! Window management.
//! wrapper for winit and gfx context creation

use std::time::Duration;

use gfx;
use gfx_core;
use imgui::{ImGui, Ui};
use imgui_gfx_renderer::{Renderer, Shaders};
use glutin as winit;
use gfx_window_glutin;

use graphics;
use screen;

/// Builds Windows
pub struct Builder {
    dimensions: Option<(u32, u32)>,
    fullscreen: Option<winit::MonitorId>,
    title: Option<String>,
    vsync: bool,
    multisampling: u16,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            dimensions: None,
            fullscreen: None,
            title: None,
            vsync: false,
            multisampling: 0,
        }
    }

    pub fn with_multisampling(mut self, multisampling: u16) -> Builder {
        self.multisampling = multisampling;
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Builder {
        self.dimensions = Some((width, height));
        self
    }

    pub fn with_fullscreen(mut self, f: winit::MonitorId) -> Builder {
        self.fullscreen = Some(f);
        self
    }

    pub fn with_title(mut self, t: String) -> Builder {
        self.title = Some(t);
        self
    }

    pub fn with_vsync(mut self, v: bool) -> Builder {
        self.vsync = v;
        self
    }

    pub fn build(self) -> Window {
        let dimensions = match self.dimensions {
            Some(d) => d,
            None => (1280, 720),
        };
        let title = match self.title {
            Some(t) => t,
            None => "Default Window".to_owned(),
        };

        let events_loop = winit::EventsLoop::new();
        let context = winit::ContextBuilder::new()
            .with_vsync(self.vsync)
            .with_multisampling(self.multisampling);
        let window_builder = winit::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dimensions.0, dimensions.1)
            .with_fullscreen(self.fullscreen);
        let (window_handle, device, factory, main_color, main_depth) =
            gfx_window_glutin::init::<graphics::ColorFormat, graphics::DepthFormat>(
                window_builder,
                context,
                &events_loop,
            );

        let renderer = graphics::Renderer::new(factory, device, main_color, main_depth);

        let window = Window {
            screen: screen::Screen::new(renderer),
            dimensions: dimensions,
            events_loop: events_loop,
            window_handle: window_handle,
            //ui_texture: None,
            //ui_color: None,
            //ui_renderer: None,
            //imgui: None,
            //mouse_state: MouseState::default(),
        };

        window
    }
}

pub struct Window {
    screen: screen::Screen,
    dimensions: (u32, u32),
    events_loop: winit::EventsLoop,
    window_handle: winit::GlWindow,

    // NOTE: UI stuff, will probably move this away later
    /*
    imgui: Option<ImGui>,
    ui_texture: Option<graphics::texture::Texture>,
    ui_color: Option<gfx_core::handle::RenderTargetView<graphics::Resources, gfx::format::Rgba8>>,
    ui_renderer: Option<Renderer<graphics::Resources>>,
    mouse_state: MouseState,
    */
}

impl Window {
    pub fn render(mut self) -> Window {
        self.screen = self.screen.render();

        use glutin::GlContext;
        self.window_handle.swap_buffers().unwrap();
        self.screen.cleanup();
        self
    }

    pub fn update(&mut self, delta: Duration) -> () {
        self.screen.update(delta);
    }

    pub fn poll_events(&mut self) -> bool {
        use winit::WindowEvent::*;
        use winit::ElementState::Pressed;
        use winit::{Event, MouseButton, MouseScrollDelta, TouchPhase};

        let mut running = true;
        /*
        let imgui = match self.imgui {
            Some(ref mut imgui) => imgui,
            None => panic!("ImGui not initialized??"),
        };
        let ref mut mouse_state = self.mouse_state;
        */

        self.events_loop.poll_events(|event| match event {
            winit::Event::WindowEvent { event, .. } => {
                match event {
                    Closed => running = false,
                    KeyboardInput { input, .. } => {
                        /*
                        use glutin::VirtualKeyCode as Key;

                        let pressed = input.state == Pressed;
                        match input.virtual_keycode {
                            Some(Key::Tab) => imgui.set_key(0, pressed),
                            Some(Key::Left) => imgui.set_key(1, pressed),
                            Some(Key::Right) => imgui.set_key(2, pressed),
                            Some(Key::Up) => imgui.set_key(3, pressed),
                            Some(Key::Down) => imgui.set_key(4, pressed),
                            Some(Key::PageUp) => imgui.set_key(5, pressed),
                            Some(Key::PageDown) => imgui.set_key(6, pressed),
                            Some(Key::Home) => imgui.set_key(7, pressed),
                            Some(Key::End) => imgui.set_key(8, pressed),
                            Some(Key::Delete) => imgui.set_key(9, pressed),
                            Some(Key::Back) => imgui.set_key(10, pressed),
                            Some(Key::Return) => imgui.set_key(11, pressed),
                            Some(Key::Escape) => imgui.set_key(12, pressed),
                            Some(Key::A) => imgui.set_key(13, pressed),
                            Some(Key::C) => imgui.set_key(14, pressed),
                            Some(Key::V) => imgui.set_key(15, pressed),
                            Some(Key::X) => imgui.set_key(16, pressed),
                            Some(Key::Y) => imgui.set_key(17, pressed),
                            Some(Key::Z) => imgui.set_key(18, pressed),
                            Some(Key::LControl) |
                            Some(Key::RControl) => imgui.set_key_ctrl(pressed),
                            Some(Key::LShift) |
                            Some(Key::RShift) => imgui.set_key_shift(pressed),
                            Some(Key::LAlt) | Some(Key::RAlt) => imgui.set_key_alt(pressed),
                            Some(Key::LWin) | Some(Key::RWin) => imgui.set_key_super(pressed),
                            _ => {}
                        }
                        */
                    }
                    MouseMoved { position: (x, y), .. } => {}, //mouse_state.pos = (x as i32, y as i32),
                    MouseInput { state, button, .. } => {
                        /*
                        match button {
                            MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                            MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                            MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                            _ => {}
                        }
                        */
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } |
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => {}, //mouse_state.wheel = y,
                    ReceivedCharacter(c) => {}, //imgui.add_input_character(c),
                    _ => (),
                    _ => (),
                }
            }
            _ => (),
        });

        running
    }

    /*
    pub fn setup_ui(&mut self) -> () {
        use gfx_core::Factory;
        let ref device = self.device;
        let ref mut factory = self.factory;

        let shaders = {
            let version = device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else {
                if version.major >= 4 {
                    Shaders::GlSl400
                } else if version.major >= 3 {
                    Shaders::GlSl130
                } else {
                    Shaders::GlSl110
                }
            }
        };

        let (_, ui_view, ui_color) = factory
            .create_render_target::<gfx::format::Rgba8>(
                self.dimensions.0 as u16,
                self.dimensions.1 as u16,
            )
            .unwrap();

        self.ui_texture = Some(
            graphics::texture::Builder::new()
                .with_view(ui_view)
                .with_dimensions(self.dimensions.0, self.dimensions.1)
                .build(factory),
        );

        let mut imgui = ImGui::init();
        let renderer =
            Renderer::init::<graphics::Factory>(&mut imgui, factory, shaders, ui_color.clone())
                .expect("Failed to initialize renderer");

        configure_keys(&mut imgui);

        self.imgui = Some(imgui);
        self.ui_color = Some(ui_color);
        self.ui_renderer = Some(renderer);
    }

    pub fn get_ui_frame(&mut self, dt: f32) -> Ui {
        let ref mut window = self.glutin_window;
        let imgui = match self.imgui {
            Some(ref mut imgui) => imgui,
            None => panic!("ImGui not initialized??"),
        };
        update_mouse(imgui, &mut self.mouse_state);

        let size_points = window.get_inner_size_points().unwrap();
        let size_pixels = window.get_inner_size_pixels().unwrap();

        imgui.frame(size_points, size_pixels, dt)
    }

    pub fn render_ui(&mut self) -> () {}
    */
}


// some imgui-rs helpers
// TODO: move these out of here
/*
fn configure_keys(imgui: &mut ImGui) {
    use imgui::ImGuiKey;

    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(
        mouse_state.pos.0 as f32 / scale.0,
        mouse_state.pos.1 as f32 / scale.1,
    );
    imgui.set_mouse_down(
        &[
            mouse_state.pressed.0,
            mouse_state.pressed.1,
            mouse_state.pressed.2,
            false,
            false,
        ],
    );
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

*/