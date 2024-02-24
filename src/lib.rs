use std::{collections::HashMap, error::Error};

use winit::{event_loop::EventLoop, window::WindowId};

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

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(e) => panic!("Error Creating EventLoop: {}", e.to_string()),
        };

        let mut builder = winit::window::WindowBuilder::new();

        let window = match builder.build(&event_loop){
            Ok(window) => window,
            Err(e) => panic!("Error creating window: {}", e.to_string()),
        };

        event_loop.run(move |event, event_loop| match event {
            // Window Events
            winit::event::Event::WindowEvent { window_id, event } => {
                match event {
                    winit::event::WindowEvent::CloseRequested => event_loop.exit(), // Close window on close request
                    _ => (),
                }
            },
            _ => (),
        })?;

        Ok(())
    }
}
