// The main.rs file exists as a Sandboxing environment for the Gazelle application.
use gazelle::{self, event::Event};

fn main() {
    let mut sandbox = match gazelle::Application::build() {
        Ok(app) => app,
        Err(e) => panic!("Error creating application"),
    };

    sandbox.run();
}
