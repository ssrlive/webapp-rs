use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(data: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &data.health_check_response;
    let mut visit_count = data.visit_count.lock().unwrap();
    let response = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
