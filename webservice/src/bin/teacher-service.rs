use actix_web::{web, App, HttpServer};
use std::io;

use webservice::db_access::init_db;
use webservice::routers::*;
use webservice::state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = init_db(&db_url).await.unwrap();

    let state = web::Data::new(AppState::new("I'm fine", db_pool));
    let app = move || {
        App::new()
            .app_data(state.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
