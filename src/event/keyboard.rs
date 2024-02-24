pub struct KeyPressed {
    keycode: i32,
    repeat_count: i32,
}

impl KeyPressed {
    pub fn create(keycode: i32, repeat_count: i32) -> Self {
        Self {
            keycode,
            repeat_count,
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
        format!("Key Pressed Event: keycode={} repeat_count={}", self.keycode, self.repeat_count)
    }
}

pub struct KeyReleased {
    keycode: i32,
}

impl KeyReleased {
    pub fn create(keycode: i32) -> Self {
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
        format!("Key Released Event: keycode={}", self.keycode)
    }
}