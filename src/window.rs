use std::fs::{File};
use std::io::Read;
use std::time::{Duration, SystemTime};
use crate::keyboard::KeyboardEvent;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::dpi::{PhysicalSize, Size};
use winit::event::DeviceEvent;

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

    pub async fn run(self) {
        println!("Starting up event loop...");
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(self.title)
            .with_maximized(true)
            .with_inner_size(Size::from(PhysicalSize::new(self.width, self.height)))
            .build(&event_loop)
            .unwrap();

        let instance = wgpu::Instance::default();

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
            .await
            .expect("Failed to find an appropriate adapter");


        // Create the logical device and command queue
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            },
            None,
        )
            .await
            .expect("Failed to create device");

        let mut shader_file = File::open("assets\\shaders\\default.wgsl").expect("Shader file not found!");
        let mut shader_content = String::new();
        shader_file.read_to_string(&mut shader_content).expect("Failed to read in glsl file!");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&shader_content)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let size = window.inner_size();

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        event_loop.run(move |event, _, control_flow| {
            let begin = SystemTime::now();
            *control_flow = ControlFlow::Wait;

            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::Key(input),
                    ..
                }  if window.has_focus() => KeyboardEvent::new(input).handle_event(),
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    // Reconfigure the surface with the new size
                    config.width = size.width;
                    config.height = size.height;
                    surface.configure(&device, &config);
                    // On macos the window needs to be redrawn manually after resizing
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let frame = surface
                        .get_current_texture()
                        .expect("Failed to acquire next swap chain texture");

                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });
                        pass.set_pipeline(&render_pipeline);
                        pass.draw(0..3, 0..1);
                    }

                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }

            println!("FPS: {}", 1.0 / begin.elapsed().unwrap().as_secs_f32());
        });
    }
}

impl Default for Window {
    fn default() -> Self {
        return Self {
            width: 1920,
            height: 1080,
            title: String::from(""),
        };
    }
}
