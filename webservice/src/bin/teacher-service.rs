use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

use webservice::routers::*;
use webservice::state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let data = web::Data::new(AppState {
        health_check_response: String::from("I'm healthy"),
        visit_count: Mutex::new(0),
        course: Mutex::new(vec![]),
    });
    let app = move || App::new().app_data(data.clone()).configure(general_routes);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
