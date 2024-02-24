use std::{collections::HashMap, f32::consts::E};

use event::window;
use winit::{event_loop::EventLoop, window::{WindowBuilder, WindowId}};

// Modules
pub mod event;

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

    pub fn run(&mut self) {
        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(e) => panic!("Error Creating EventLoop: {}", e.to_string()),
        };

        let mut builder = winit::window::WindowBuilder::new();

        let window = match builder.build(&event_loop){
            Ok(window) => window,
            Err(e) => panic!("Error creating window: {}", e.to_string()),
        };
    }
}
