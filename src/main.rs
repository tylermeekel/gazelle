// The main.rs file exists as a Sandboxing environment for the Gazelle application.
use gazelle;

#[tokio::main]
async fn main() {
    let mut sandbox = gazelle::Application::build().unwrap();

    match sandbox.run().await {
        Ok(_) => (),
        Err(e) => panic!("{}", e.to_string()),
    };
}
