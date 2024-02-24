use super::EventCategoryFlag;

pub struct KeyPressed {
    pub keycode: i32,
    pub repeat_count: i32,
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

    fn is_in_category(&self, category: EventCategoryFlag) -> bool {
        self.category_flags().contains(category)
    }
}