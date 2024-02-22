// The main.rs file exists as a Sandboxing environment for the Gazelle application.
use gazelle;

fn main() {
    let mut sandbox = gazelle::Application::build().unwrap();

    sandbox.run();
}
