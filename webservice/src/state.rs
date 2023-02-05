use crate::dbaccess::dbinit::Db;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<i32>,
    pub db: Db,
}

impl AppState {
    pub fn new(response: &str, db: Db) -> Self {
        AppState {
            health_check_response: String::from(response),
            visit_count: Mutex::new(0),
            db,
        }
    }
}
