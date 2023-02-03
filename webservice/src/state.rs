use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<i32>,
    pub db: sqlx::PgPool,
}

impl AppState {
    pub fn new(response: &str, db: sqlx::PgPool) -> Self {
        AppState {
            health_check_response: String::from(response),
            visit_count: Mutex::new(0),
            db,
        }
    }
}
