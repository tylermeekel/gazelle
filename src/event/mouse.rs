pub struct MouseMoved {
    x: u32,
    y: u32,
    is_handled: bool,
}

impl MouseMoved {
    pub fn create(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            is_handled: false,
        }
    }
}

impl super::Event for MouseMoved {
    fn event_type(&self) -> super::EventType {
        super::EventType::MouseMoved
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::MouseMoved | super::EventCategoryFlag::Input
    }

    fn description(&self) -> String {
        format!("Mouse Moved Event: x={} y={}", self.x, self.y)
    }

    fn is_handled(&self) -> bool {
        self.is_handled
    }

    fn set_handled(&mut self, was_handled: bool) {
        self.is_handled = was_handled
    }
}

pub struct MouseScrolled {
    is_handled: bool,
}

impl MouseScrolled {
    pub fn create() -> Self {
        Self { is_handled: false }
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

    fn is_handled(&self) -> bool {
        self.is_handled
    }

    fn set_handled(&mut self, was_handled: bool) {
        self.is_handled = was_handled
    }
}

pub struct MouseButtonPressed {
    button: winit::event::MouseButton,
    is_handled: bool,
}

impl MouseButtonPressed {
    pub fn create(button: winit::event::MouseButton) -> Self {
        Self {
            button,
            is_handled: false,
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

    fn is_handled(&self) -> bool {
        self.is_handled
    }

    fn set_handled(&mut self, was_handled: bool) {
        self.is_handled = was_handled
    }
}

pub struct MouseButtonReleased {
    button: winit::event::MouseButton,
    is_handled: bool,
}

impl MouseButtonReleased {
    pub fn create(button: winit::event::MouseButton) -> Self {
        Self {
            button,
            is_handled: false,
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

    fn is_handled(&self) -> bool {
        self.is_handled
    }

    fn set_handled(&mut self, was_handled: bool) {
        self.is_handled = was_handled
    }
}
