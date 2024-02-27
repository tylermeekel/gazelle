use bitflags::bitflags;

// Submodules
pub mod application;
pub mod keyboard;
pub mod mouse;
pub mod window;

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
    // TODO: Implement Gamepad Events
}

// Helper function for declaring bit flags
// Shifts a 1 left n times
const fn bit(n_shifts: u8) -> u32 {
    1 << n_shifts
}

// Declare Bitflags for Event categories, so that we can support
// having multiple categories per event type
bitflags! {
    pub struct EventCategoryFlag: u32 {
        const Application = bit(0);
        const Keyboard = bit(1);
        const MouseMoved = bit(2);
        const MouseButton = bit(3);
        const Input = bit(4);
        const Window = bit(5);
    }
}

pub trait Event {
    fn event_type(&self) -> EventType;
    fn category_flags(&self) -> EventCategoryFlag;
    fn description(&self) -> String;
    fn is_in_category(&self, category: EventCategoryFlag) -> bool {
        self.category_flags().contains(category)
    }
    fn is_handled(&self) -> bool;
}
