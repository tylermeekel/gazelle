use crate::event::{self, Event};

// Modules
pub mod stack;

// Layers break up chunks of code in order to control
// error handling more precisely
pub trait Layer {
    fn on_attach(&self); // Called when the Layer is attached to the application
    fn on_detach(&self); // Called when the Layer is detached from the application
    fn on_update(&self); // Called on every application update cycle
    fn on_event(&self, event: &Box<dyn Event>); // Called when sent an event
    fn debug_name(&self) -> String; // used for debugging only

    // TODO: add enabled status, to hide from layer stack
}
