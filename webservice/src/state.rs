use crate::models::Course;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<i32>,
    pub courses: Mutex<Vec<Course>>,
}

impl AppState {
    pub fn new(response: &str) -> Self {
        AppState {
            health_check_response: String::from(response),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        }
    }
}
