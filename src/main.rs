// The main.rs file exists as a Sandboxing environment for the Gazelle application.
use gazelle::application;

fn main() {
    let mut sandbox = application::Application {

    };

    sandbox.run();
}
