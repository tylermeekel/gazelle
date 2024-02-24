use winit::keyboard::SmolStr;

pub struct KeyPressed {
    keycode: winit::keyboard::Key<SmolStr>,
    repeat: bool,
}

impl KeyPressed {
    pub fn create(keycode: winit::keyboard::Key<SmolStr>, repeat: bool) -> Self {
        Self {
            keycode,
            repeat,
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
        format!("Key Pressed Event: key={:?} repeat={}", self.keycode, self.repeat)
    }
}

pub struct KeyReleased {
    keycode: winit::keyboard::Key<SmolStr>,
}

impl KeyReleased {
    pub fn create(keycode: winit::keyboard::Key<SmolStr>) -> Self {
        Self {
            keycode
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
}