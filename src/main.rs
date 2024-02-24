// The main.rs file exists as a Sandboxing environment for the Gazelle application.
use gazelle::{self, event::Event};

fn main() {
    let kpe = gazelle::event::keyboard::KeyPressed {
        keycode: 1,
        repeat_count: 1,
    };

    println!("{}", kpe.is_in_category(gazelle::event::EventCategoryFlag::Input));
    println!("{}", kpe.is_in_category(gazelle::event::EventCategoryFlag::MouseButton));
}
