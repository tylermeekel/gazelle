use winit::keyboard::SmolStr;

pub struct KeyPressed {
    keycode: winit::keyboard::Key<SmolStr>,
    repeat: bool,
    is_handled: bool,
}

impl KeyPressed {
    pub fn create(keycode: winit::keyboard::Key<SmolStr>, repeat: bool) -> Self {
        Self {
            keycode,
            repeat,
            is_handled: false,
        }
    }
}

impl super::Event for KeyPressed {
    fn event_type(&self) -> super::EventType {
        super::EventType::KeyPressed
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::Keyboard | super::EventCategoryFlag::Input
    }

    fn description(&self) -> String {
        format!(
            "Key Pressed Event: key={:?} repeat={}",
            self.keycode, self.repeat
        )
    }

    fn is_handled(&self) -> bool {
        self.is_handled
    }
}

pub struct KeyReleased {
    keycode: winit::keyboard::Key<SmolStr>,
    is_handled: bool,
}

impl KeyReleased {
    pub fn create(keycode: winit::keyboard::Key<SmolStr>) -> Self {
        Self {
            keycode,
            is_handled: false,
        }
    }
}

impl super::Event for KeyReleased {
    fn event_type(&self) -> super::EventType {
        super::EventType::KeyReleased
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::Keyboard | super::EventCategoryFlag::Input
    }

    fn description(&self) -> String {
        format!("Key Released Event: keycode={:?}", self.keycode)
    }

    fn is_handled(&self) -> bool {
        self.is_handled
    }
}
