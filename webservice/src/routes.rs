use crate::handlers::{course::*, general::*, teacher::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course_handler))
            .route("/{teacher_id}", web::get().to(get_courses_of_teacher))
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_course_detail),
            )
            .route(
                "/{teacher_id}/{course_id}",
                web::put().to(update_course_handler),
            )
            .route(
                "/{teacher_id}/{course_id}",
                web::delete().to(delete_course_handler),
            ),
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teachers")
            .route("/", web::post().to(new_teacher_handler))
            .route("/", web::get().to(get_teachers_handler))
            .route("/{teacher_id}", web::get().to(get_teacher_handler))
            .route("/{teacher_id}", web::put().to(update_teacher_handler))
            .route("/{teacher_id}", web::delete().to(delete_teacher_handler)),
    );
}
