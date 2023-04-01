use std::fs::{File};
use std::io::Read;
use std::num::NonZeroU32;
use std::time::{SystemTime};

use raw_window_handle::HasRawWindowHandle;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::SwapInterval;

use glutin_winit;
use glutin_winit::DisplayBuilder;
use glutin_winit::GlWindow;

use crate::keyboard::KeyboardEvent;

use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use winit::dpi::PhysicalSize;
use winit::dpi::Size;
use winit::event::DeviceEvent;
use crate::renderer::Renderer;

pub struct Window {
    width: u16,
    height: u16,
    title: String,
}

impl Window {
    pub fn new(width: u16, height: u16, title: String) -> Self {
        return Self {
            width,
            height,
            title,
        };
    }

    pub fn run(self) {
        println!("Starting up event loop...");
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(self.title)
            .with_maximized(true)
            .with_inner_size(Size::from(PhysicalSize::new(self.width, self.height)));

        // The template will match only the configurations supporting rendering
        // to windows.
        //
        // XXX We force transparency only on macOS, given that EGL on X11 doesn't
        // have it, but we still want to show window. The macOS situation is like
        // that, because we can query only one config at a time on it, but all
        // normal platforms will return multiple configs, so we can find the config
        // with transparency ourselves inside the `reduce`.
        let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (mut window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                // Find the config with the maximum number of samples, so our triangle will
                // be smooth.
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        println!("Picked a config with {} samples", gl_config.num_samples());

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        // XXX The display could be obtained from the any object created by it, so we
        // can query it from the config.
        let gl_display = gl_config.display();

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(raw_window_handle);

        let mut not_current_gl_context = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display.create_context(&gl_config, &fallback_context_attributes).unwrap_or_else(
                    |_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    },
                )
            })
        });

        let mut state = None;
        let mut renderer = None;
        event_loop.run(move |event, window_target, control_flow| {
            let begin_frame = SystemTime::now();
            control_flow.set_wait();
            match event {
                Event::Resumed => {
                    let window = window.take().unwrap_or_else(|| {
                        let window_builder = WindowBuilder::new().with_transparent(true);
                        glutin_winit::finalize_window(window_target, window_builder, &gl_config)
                            .unwrap()
                    });

                    let attrs = window.build_surface_attributes(Default::default());
                    let gl_surface = unsafe {
                        gl_config.display().create_window_surface(&gl_config, &attrs).unwrap()
                    };

                    // Make it current.
                    let gl_context =
                        not_current_gl_context.take().unwrap().make_current(&gl_surface).unwrap();

                    // The context needs to be current for the Renderer to set up shaders and
                    // buffers. It also performs function loading, which needs a current context on
                    // WGL.
                    renderer.get_or_insert_with(|| Renderer::new(&gl_display));

                    // Try setting vsync.
                    if let Err(res) = gl_surface
                        .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                    {
                        eprintln!("Error setting vsync: {res:?}");
                    }

                    assert!(state.replace((gl_context, gl_surface, window)).is_none());
                }
                Event::Suspended => {
                    // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
                    // the window back to the system.
                    let (gl_context, ..) = state.take().unwrap();
                    assert!(not_current_gl_context
                        .replace(gl_context.make_not_current().unwrap())
                        .is_none());
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            // Some platforms like EGL require resizing GL surface to update the size
                            // Notable platforms here are Wayland and macOS, other don't require it
                            // and the function is no-op, but it's wise to resize it for portability
                            // reasons.
                            if let Some((gl_context, gl_surface, _)) = &state {
                                gl_surface.resize(
                                    gl_context,
                                    NonZeroU32::new(size.width).unwrap(),
                                    NonZeroU32::new(size.height).unwrap(),
                                );
                                let renderer = renderer.as_ref().unwrap();
                                renderer.resize(size.width as i32, size.height as i32);
                            }
                        }
                    }
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    _ => (),
                },
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::Key(key) => {
                        KeyboardEvent::new(key).handle_event();
                    }
                    _ => (),
                }
                Event::RedrawEventsCleared => {
                    if let Some((gl_context, gl_surface, window)) = &state {
                        let renderer = renderer.as_ref().unwrap();
                        renderer.draw();
                        window.request_redraw();

                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                }
                _ => (),
            }

            println!("FPS: {}", 1.0 / begin_frame.elapsed().unwrap().as_secs_f32())
        });
    }
}