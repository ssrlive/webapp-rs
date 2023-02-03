use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &state.health_check_response;
    let mut visit_count = state.visit_count.lock().unwrap();
    let response = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
