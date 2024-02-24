use std::{collections::HashMap, error::Error, f32::consts::E};

use winit::{event_loop::EventLoop, window::WindowId};

use crate::event::Event;

// Modules
pub mod event;
pub mod logging;

struct WindowState;

// Application is the entry point for a Gazelle application
pub struct Application {
    windows: HashMap<WindowId, WindowState>
}

#[derive(Debug)]
pub enum ApplicationBuildError {

}

impl Application {
    pub fn build() -> Result<Self, ApplicationBuildError> {
        Ok(Application{
            windows: HashMap::new()
        })
    }

    // Take events from the event loop and handle them
    fn handle_event(&self, e: winit::event::WindowEvent) {
        use winit::event::WindowEvent::*;

        let gz_event: Option<Box<dyn Event>> = match e {
            CursorMoved { position, .. } => {
                Some(Box::new(event::mouse::MouseMoved::create(position.x as u32, position.y as u32)))
            },
            KeyboardInput { event, .. } => {
                match event.state {
                    winit::event::ElementState::Pressed => {
                        Some(Box::new(crate::event::keyboard::KeyPressed::create(event.logical_key, event.repeat)))
                    },
                    winit::event::ElementState::Released => {
                        Some(Box::new(crate::event::keyboard::KeyReleased::create(event.logical_key)))
                    },
                }
            },
            MouseInput { state, button, .. } => {
                match state {
                    winit::event::ElementState::Pressed => Some(Box::new(event::mouse::MouseButtonPressed::create(button))),
                    winit::event::ElementState::Released => Some(Box::new(event::mouse::MouseButtonReleased::create(button))),
                }
            },
            MouseWheel { .. } => Some(Box::new(event::mouse::MouseScrolled::create())),
            _ => None,
        };

        // TODO: Implement an event dispatcher
        match gz_event {
            Some(event) => logging::log_core(event.description()),
            None => (),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(e) => panic!("Error Creating EventLoop: {}", e.to_string()),
        };

        let builder = winit::window::WindowBuilder::new();

        let window = match builder.build(&event_loop){
            Ok(window) => window,
            Err(e) => panic!("Error creating window: {}", e.to_string()),
        };

        event_loop.run(move |event, event_loop| match event {
            // Window Events
            winit::event::Event::WindowEvent { window_id, event } => {
                match event {
                    winit::event::WindowEvent::CloseRequested => event_loop.exit(), // exit the event loop when window close is requested
                    event => self.handle_event(event), // handle the event otherwise
                }
            },
            _ => (),
        })?;

        Ok(())
    }
}
