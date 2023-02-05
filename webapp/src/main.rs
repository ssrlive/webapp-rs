use actix_web::{web, App, HttpServer};
use tera::Tera;
use webapp::{routes::app_config, state::AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let svc_host = std::env::var("FRONTEND_HOST_PORT").expect("FRONTEND_HOST_PORT must be set");
    println!("Starting server at {svc_host}");

    let backend_host =
        std::env::var("FRONTEND_TO_BACKEND_URL").expect("FRONTEND_TO_BACKEND_URL must be set");
    let state = AppState::new(backend_host);
    let state = web::Data::new(state);

    HttpServer::new(move || {
        let t = Tera::new(concat!(std::env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .app_data(state.clone())
            .app_data(web::Data::new(t))
            .configure(app_config)
    })
    .bind(svc_host)?
    .run()
    .await
}
