pub struct AppState {
    pub backend_url: String,
}

impl AppState {
    pub fn new(backend_url: String) -> Self {
        Self { backend_url }
    }
}
