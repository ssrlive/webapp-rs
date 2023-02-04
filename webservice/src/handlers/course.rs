use crate::models::course::CreateCourse;
use crate::state::AppState;
use crate::{dbaccess::course::*, errors::Result};
use actix_web::{web, HttpResponse};

pub async fn new_course_handler(
    state: web::Data<AppState>,
    course: web::Json<CreateCourse>,
) -> Result<HttpResponse> {
    let courses = post_course_db(&state.db, &course).await?;
    Ok(HttpResponse::Ok().json(&courses))
}

pub async fn get_courses_of_teacher(
    state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse> {
    let teacher_id = i64::try_from(params.into_inner()).unwrap();
    get_courses_of_teacher_db(&state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(&courses))
}

pub async fn get_course_detail(
    state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse> {
    let (teacher_id, course_id) = params.into_inner();
    let teacher_id = i64::try_from(teacher_id).unwrap();
    let course_id = i64::try_from(course_id).unwrap();
    get_course_details_db(&state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(&course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::course::Course;
    use actix_web::http::StatusCode;
    use sqlx::postgres::PgPoolOptions;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let course = web::Json(Course::new(1, "Math").into());
        let state = AppState::new("I'm healthy", db_pool);
        let state = web::Data::new(state);
        let response = new_course_handler(state, course).await.unwrap();
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
        let response = get_courses_of_teacher(state, teacher_id).await.unwrap();
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
        let response = get_course_detail(state, params).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
