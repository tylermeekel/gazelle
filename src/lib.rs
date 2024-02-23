// Modules
pub mod event;

// Application is the entry point for a Gazelle application
pub struct Application {

}

#[derive(Debug)]
pub enum ApplicationBuildError {

}

impl Application {
    pub fn build() -> Result<Self, ApplicationBuildError> {
        Ok(Application{})
    }

    pub fn run(&mut self) {
        println!("This is running!")
    }
}
