use bitflags::bitflags;

// Submodules
pub mod application;
pub mod window;
pub mod mouse;
pub mod keyboard;

// Describes different supported event types
pub enum EventType {
    // Window Event Types
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    // App Event Types
    AppTick,
    AppUpdate,
    AppRender,
    // Key Event Types
    KeyPressed,
    KeyReleased,
    // Mouse Event Types
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,

    // TODO: Implement Gamepad Support
}

bitflags! {
    pub struct EventCategoryFlag: u32 {
        const Application = 0b00000001;
        const Keyboard = 0b00000010;
        const MouseMoved = 0b00000100;
        const MouseButton = 0b00001000;
        const Input = 0b00010000;
        const Window = 0b00100000;
    } 
}

pub trait Event {
    fn event_type(&self) -> EventType;
    fn category_flags(&self) -> EventCategoryFlag;
    fn description(&self) -> String;
    fn is_in_category(&self, ecf: EventCategoryFlag) -> bool;
}
