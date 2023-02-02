use actix_web::{web, App, HttpServer};
use std::io;

use webservice::routers::*;
use webservice::state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let data = web::Data::new(AppState::new("I'm fine"));
    let app = move || {
        App::new()
            .app_data(data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
