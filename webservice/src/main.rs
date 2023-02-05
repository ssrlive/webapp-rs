use actix_web::{web, App, HttpServer};
use std::io;
use webservice::dbaccess::dbinit::db_initialize;
use webservice::errors::ServiceError;
use webservice::routes::*;
use webservice::state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let svc_addr = std::env::var("BACKEND_HOST_PORT").expect("BACKEND_HOST_PORT must be set");
    let sql_file = std::env::var("SQL_FILE_PATH").ok();
    if sql_file.is_none() {
        println!("SQL_FILE_PATH not set, skipping database initialization");
    }

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = db_initialize(&db_url, sql_file).await.unwrap();

    let state = web::Data::new(AppState::new("I'm fine", db_pool));
    let app = move || {
        App::new()
            .app_data(state.clone())
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|e, _| ServiceError::InvalidInput(e.to_string()).into()),
            )
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind(&svc_addr)?.run().await
}
