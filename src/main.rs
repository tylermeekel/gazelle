// The main.rs file exists as a Sandboxing environment for the Gazelle application.
use gazelle::{self, layer::Layer};

struct ExampleLayer {}

impl Layer for ExampleLayer {
    fn on_attach(&self) {
        todo!()
    }

    fn on_detach(&self) {
        todo!()
    }

    fn on_update(&self) {
        println!("Update");
    }

    fn on_event(&self, event: &Box<dyn gazelle::event::Event>) {
        println!("Received Event: {}", event.description());
    }

    fn debug_name(&self) -> String {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let mut sandbox = gazelle::Application::build().unwrap();

    let exlayer = ExampleLayer {};

    sandbox.push_layer(Box::new(exlayer));

    match sandbox.run().await {
        Ok(_) => (),
        Err(e) => panic!("{}", e.to_string()),
    };
}
