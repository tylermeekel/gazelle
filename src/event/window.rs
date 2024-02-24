// TODO: Implement Remaining Window Events

pub struct WindowClose;

impl WindowClose {
    pub fn create() -> Self {
        Self
    }
}

impl super::Event for WindowClose {
    fn event_type(&self) -> super::EventType {
        super::EventType::WindowClose
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::Application | super::EventCategoryFlag::Window
    }

    fn description(&self) -> String {
        String::from("Window Close Event")
    }
}

pub struct WindowResize {
    h: i32,
    w: i32,
}

impl WindowResize {
    pub fn create(h: i32, w: i32) -> Self {
        Self {
            h,
            w,
        }
    }
}

impl super::Event for WindowResize {
    fn event_type(&self) -> super::EventType {
        super::EventType::WindowResize
    }

    fn category_flags(&self) -> super::EventCategoryFlag {
        super::EventCategoryFlag::Window
    }

    fn description(&self) -> String {
        format!("Window Resize Event: h={} w={}", self.h, self.w)
    }
}