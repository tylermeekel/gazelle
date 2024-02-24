use std::{collections::HashMap, error::Error};

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
    // TODO: Implement an event dispatcher
    fn handle_event(&self, e: winit::event::WindowEvent) {
        use winit::event::WindowEvent::*;

        match e {
            CursorMoved { position, .. } => {
                let cme = event::mouse::MouseMoved::create(position.x as u32, position.y as u32);
                logging::log_core(cme.description())
            },
            KeyboardInput { event, .. } => {
                match event.state {
                    winit::event::ElementState::Pressed => {
                        let kpe = crate::event::keyboard::KeyPressed::create(event.logical_key, 1);
                        logging::log_core(kpe.description());
                    },
                    winit::event::ElementState::Released => {
                        let kre = crate::event::keyboard::KeyPressed::create(event.logical_key, 1);
                        logging::log_core(kre.description());
                    },
                }
            }
            _ => (),
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
                self.handle_event(event);
            },
            _ => (),
        })?;

        Ok(())
    }
}
