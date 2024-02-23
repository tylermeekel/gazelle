// Submodules
mod application;
mod window;
mod mouse;
mod keyboard;

// Helper function for creating bit flags
const fn BIT(n: u32) -> isize {
    1 << n
}

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

pub enum EventCategoryFlag {
    EventCategoryApplication = 0,
    EventCategoryInput = BIT(0),
    EventCategoryKeyboard = BIT(1),
    EventCategoryMouse = BIT(2),
    EventCategoryMouseButton = BIT(3),

    // TODO: Implement Gamepad Category
}

pub trait Event {
    fn get_event_category_flags(&self) -> isize;
}
