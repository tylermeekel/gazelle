use std::{borrow::Cow, collections::HashMap, error::Error};

use winit::{event_loop::EventLoop, window::WindowId};

use crate::event::Event;

// Modules
pub mod event;
pub mod layer;
pub mod logging;

struct WindowState;

// Application is the entry point for a Gazelle application
pub struct Application {
    layer_stack: layer::stack::LayerStack,
}

#[derive(Debug)]
pub enum ApplicationBuildError {}

impl Application {
    pub fn push_layer(&mut self, new_layer: Box<dyn layer::Layer>) {
        self.layer_stack.push_layer(new_layer)
    }

    pub fn build() -> Result<Self, ApplicationBuildError> {
        Ok(Application {
            layer_stack: layer::stack::LayerStack::create(),
        })
    }

    // Take events from the event loop and handle them
    fn handle_event(&self, e: winit::event::WindowEvent) {
        use winit::event::WindowEvent::*;

        // Create gazelle event based on the window event, this allows us to pick and choose specific events to implement
        // as well as how we want to implement them
        let gz_event: Option<Box<dyn Event>> = match e {
            CursorMoved { position, .. } => Some(Box::new(event::mouse::MouseMoved::create(
                position.x as u32,
                position.y as u32,
            ))),
            // Keyboard Inputs
            KeyboardInput { event, .. } => match event.state {
                winit::event::ElementState::Pressed => Some(Box::new(
                    crate::event::keyboard::KeyPressed::create(event.logical_key, event.repeat),
                )),
                winit::event::ElementState::Released => Some(Box::new(
                    crate::event::keyboard::KeyReleased::create(event.logical_key),
                )),
            },
            // Mouse Inputs
            MouseInput { state, button, .. } => match state {
                winit::event::ElementState::Pressed => {
                    Some(Box::new(event::mouse::MouseButtonPressed::create(button)))
                }
                winit::event::ElementState::Released => {
                    Some(Box::new(event::mouse::MouseButtonReleased::create(button)))
                }
            },
            // Mouse Scroll Input
            // TODO: Add MouseScrollDelta to MouseScrolled event type
            MouseWheel { .. } => Some(Box::new(event::mouse::MouseScrolled::create())),
            _ => None,
        };

        match gz_event {
            Some(event) => {
                for layer in self.layer_stack.layers.iter() {
                    layer.on_event(&event); // send event to layer
                    if event.is_handled() {
                        break; // break out of the loop if event has been handled to avoid unnecessary calls
                    }
                }
            }
            None => (),
        }
    }

    // render to the surface
    fn render(
        &self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pipeline: &wgpu::RenderPipeline,
    ) {
        // Get current frame
        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // create winit event loop
        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(e) => panic!("Error Creating EventLoop: {}", e.to_string()),
        };

        let builder = winit::window::WindowBuilder::new();

        // build window
        let window = match builder.build(&event_loop) {
            Ok(window) => window,
            Err(e) => panic!("Error creating window: {}", e.to_string()),
        };

        let mut size = window.inner_size();

        // Create the WGPU instance
        let instance = wgpu::Instance::default();

        // Create the surface for the window
        let surface = instance.create_surface(&window).unwrap();

        // Create adapter based on hardware
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance, // Tries to use the dedicated GPU, but reverts to IGPU if not available
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Request connection to the physical device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        // Create the render pipeline based on the device connection
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                // Set vertex shader state
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                // Set fragment shader state
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);

        // run the event loop, handling events
        event_loop.run(move |event, event_loop| {
            let _ = (&instance, &adapter, &shader, &pipeline_layout);

            // Call the on_update method for each layer attached
            for layer in self.layer_stack.layers.iter() {
                layer.on_update();
            }

            match event {
                // Window Events
                winit::event::Event::WindowEvent { window_id, event } => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => event_loop.exit(),
                        winit::event::WindowEvent::RedrawRequested => {
                            self.render(&surface, &device, &queue, &render_pipeline)
                        } // exit the event loop when window close is requested
                        event => self.handle_event(event), // handle the event otherwise
                    }
                }
                _ => (),
            }
        })?;

        Ok(())
    }
}
