pub struct MouseMoved {
    x: u32,
    y: u32,
}

impl MouseMoved {
    pub fn create(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl super::Event for MouseMoved{
    fn event_type(&self) -> super::EventType {
        super::EventType::MouseMoved
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::MouseMoved | super::EventCategoryFlag::Input
    }

    fn description(&self) -> String {
        format!("Mouse Moved Event: x={} y={}", self.x, self.y)
    }
}

pub struct MouseScrolled;

impl MouseScrolled {
    pub fn create() -> Self {
        Self
    }
}

impl super::Event for MouseScrolled {
    fn event_type(&self) -> super::EventType {
        super::EventType::MouseScrolled
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::MouseMoved | super::EventCategoryFlag::Input
    }

    fn description(&self) -> String {
        format!("Mouse Scrolled Event")
    }
}

pub struct MouseButtonPressed {
    button: winit::event::MouseButton,
}

impl MouseButtonPressed {
    pub fn create(button: winit::event::MouseButton) -> Self {
        Self {
            button
        }
    }
}

impl super::Event for MouseButtonPressed {
    fn event_type(&self) -> super::EventType {
        super::EventType::MouseButtonPressed
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::MouseButton | super::EventCategoryFlag::Input
    }

    fn description(&self) -> String {
        format!("Mouse Button Pressed Event: button={:?}", self.button)
    }
}

pub struct MouseButtonReleased {
    button: winit::event::MouseButton,
}

impl MouseButtonReleased {
    pub fn create(button: winit::event::MouseButton) -> Self {
        Self {
            button,
        }
    }
}

impl super::Event for MouseButtonReleased {
    fn event_type(&self) -> super::EventType {
        super::EventType::MouseButtonReleased
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::MouseButton | super::EventCategoryFlag::Input
    }

    fn description(&self) -> String {
        format!("Mouse Button Released Event: button={:?}", self.button)
    }
}