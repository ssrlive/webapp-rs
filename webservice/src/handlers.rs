use crate::db_access::*;
use crate::models::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &state.health_check_response;
    let mut visit_count = state.visit_count.lock().unwrap();
    let response = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course_handler(
    state: web::Data<AppState>,
    course: web::Json<Course>,
) -> HttpResponse {
    let courses = post_course_db(&state.db, &course).await.unwrap();
    HttpResponse::Ok().json(&courses)
}

pub async fn get_courses_of_teacher(
    state: web::Data<AppState>,
    params: web::Path<usize>,
) -> HttpResponse {
    let teacher_id = i64::try_from(params.into_inner()).unwrap();
    let courses = get_courses_of_teacher_db(&state.db, teacher_id)
        .await
        .unwrap();
    HttpResponse::Ok().json(&courses)
}

pub async fn get_course_detail(
    state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (teacher_id, course_id) = params.into_inner();
    let teacher_id = i64::try_from(teacher_id).unwrap();
    let course_id = i64::try_from(course_id).unwrap();
    let course = get_course_details_db(&state.db, teacher_id, course_id)
        .await
        .unwrap();
    HttpResponse::Ok().json(&course)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use sqlx::postgres::PgPoolOptions;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let course = web::Json(Course::new(1, "Math".to_string()));
        let state = AppState::new("I'm healthy", db_pool);
        let state = web::Data::new(state);
        let response = new_course_handler(state, course).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_of_teacher() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let state = AppState::new("I'm healthy", db_pool);
        let state = web::Data::new(state);
        let teacher_id = web::Path::from(1);
        let response = get_courses_of_teacher(state, teacher_id).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let state = AppState::new("I'm healthy", db_pool);
        let state = web::Data::new(state);
        let params = web::Path::from((1, 1));
        let response = get_course_detail(state, params).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
