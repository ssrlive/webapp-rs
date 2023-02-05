use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;
use crate::{dbaccess::teacher::*, errors::Result};
use actix_web::{web, HttpResponse};

pub async fn new_teacher_handler(
    state: web::Data<AppState>,
    teacher: web::Json<CreateTeacher>,
) -> Result<HttpResponse> {
    let teacher = teacher.into_inner();
    let teacher = create_teacher_db(&state.db, teacher).await?;
    Ok(HttpResponse::Ok().json(teacher))
}

pub async fn get_teachers_handler(state: web::Data<AppState>) -> Result<HttpResponse> {
    let teachers = get_teachers_db(&state.db).await?;
    Ok(HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_handler(
    state: web::Data<AppState>,
    teacher_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let teacher_id = teacher_id.into_inner();
    let teacher = get_teacher_db(&state.db, teacher_id).await?;
    Ok(HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_handler(
    state: web::Data<AppState>,
    teacher_id: web::Path<i64>,
    teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResponse> {
    let teacher_id = teacher_id.into_inner();
    let teacher = teacher.into_inner();
    let teacher = update_teacher_db(&state.db, teacher_id, teacher).await?;
    Ok(HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher_handler(
    state: web::Data<AppState>,
    teacher_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let teacher_id = teacher_id.into_inner();
    let teacher = delete_teacher_db(&state.db, teacher_id).await?;
    Ok(HttpResponse::Ok().json(teacher))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dbaccess::dbinit::db_initialize,
        models::teacher::{CreateTeacher, Teacher},
    };
    use actix_web::{http::StatusCode, web::Data};

    async fn build_test_env() -> Data<AppState> {
        dotenv::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = db_initialize(&db_url, None).await.unwrap();
        let state = AppState::new("I'm healthy", db_pool);
        web::Data::new(state)
    }

    #[actix_rt::test]
    async fn test_new_teacher() {
        let state = build_test_env().await;
        let teacher = Teacher::new("John Doe", "https://example.com", "I'm a teacher");
        let teacher = web::Json(CreateTeacher::from(teacher));
        let response = new_teacher_handler(state, teacher).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_teachers() {
        let state = build_test_env().await;
        let response = get_teachers_handler(state).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_teacher() {
        let state = build_test_env().await;
        let teacher_id = web::Path::from(1);
        let response = get_teacher_handler(state, teacher_id).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[ignore = "Perhaps the teacher is not exist"]
    #[actix_rt::test]
    async fn test_update_teacher() {
        let state = build_test_env().await;
        let teacher_id = web::Path::from(1);
        let teacher = Teacher::new("Tom Smith", "https://example.com", "I'm a teacher");
        let teacher = web::Json(UpdateTeacher::from(teacher));
        let response = update_teacher_handler(state, teacher_id, teacher)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[ignore = "Perhaps the teacher is not exist"]
    #[actix_rt::test]
    async fn test_delete_teacher() {
        let state = build_test_env().await;
        let teacher_id = web::Path::from(1);
        let response = delete_teacher_handler(state, teacher_id).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
