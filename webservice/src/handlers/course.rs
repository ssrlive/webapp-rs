use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;
use crate::{dbaccess::course::*, errors::Result};
use actix_web::{web, HttpResponse};

pub async fn new_course_handler(
    state: web::Data<AppState>,
    course: web::Json<CreateCourse>,
) -> Result<HttpResponse> {
    let courses = post_course_db(&state.db, course.try_into()?).await?;
    Ok(HttpResponse::Ok().json(courses))
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

pub async fn update_course_handler(
    state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
    course: web::Json<UpdateCourse>,
) -> Result<HttpResponse> {
    let (teacher_id, course_id) = params.into_inner();
    let teacher_id = i64::try_from(teacher_id).unwrap();
    let course_id = i64::try_from(course_id).unwrap();
    update_course_db(&state.db, teacher_id, course_id, course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(&course))
}

pub async fn delete_course_handler(
    state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse> {
    let (teacher_id, course_id) = params.into_inner();
    let teacher_id = i64::try_from(teacher_id).unwrap();
    let course_id = i64::try_from(course_id).unwrap();
    delete_course_db(&state.db, teacher_id, course_id)
        .await
        .map(|_| HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::course::Course;
    use actix_web::{http::StatusCode, web::Data};
    use sqlx::postgres::PgPoolOptions;

    async fn build_test_env() -> Data<AppState> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let state = AppState::new("I'm healthy", db_pool);
        web::Data::new(state)
    }

    #[actix_rt::test]
    async fn post_course_test() {
        let state = build_test_env().await;
        let course = web::Json(Course::new(1, "Math").into());
        let response = new_course_handler(state, course).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_of_teacher() {
        let state = build_test_env().await;
        let teacher_id = web::Path::from(1);
        let response = get_courses_of_teacher(state, teacher_id).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        let state = build_test_env().await;
        let params = web::Path::from((1, 1));
        let response = get_course_detail(state, params).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[ignore = "perhaps the course is not exist"]
    #[actix_rt::test]
    async fn update_course_test() {
        let state = build_test_env().await;
        let params = web::Path::from((1, 7));
        let course = web::Json(Course::new(1, "Chinese").into());
        let response = update_course_handler(state, params, course).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[ignore = "perhaps the course is not exist"]
    #[actix_rt::test]
    async fn delete_course_test() {
        let state = build_test_env().await;
        let params = web::Path::from((1, 22));
        let response = delete_course_handler(state, params).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
