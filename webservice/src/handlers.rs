use crate::models::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(data: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &data.health_check_response;
    let mut visit_count = data.visit_count.lock().unwrap();
    let response = format!("{health_check_response} {visit_count} times");
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course_handler(
    data: web::Data<AppState>,
    course: web::Json<Course>,
) -> HttpResponse {
    let count = data
        .course
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.teacher_id == course.teacher_id)
        .collect::<Vec<_>>()
        .len();
    let new_course = Course {
        id: Some(count as i32 + 1),
        teacher_id: course.teacher_id,
        name: course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    data.course.lock().unwrap().push(new_course.clone());
    HttpResponse::Ok().json(&new_course)
}

#[actix_rt::test]
async fn post_course_test() {
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    let course = web::Json(Course {
        id: None,
        teacher_id: 1,
        name: "Math".to_string(),
        time: None,
    });
    let data = web::Data::new(AppState {
        health_check_response: String::from("I'm healthy"),
        visit_count: Mutex::new(0),
        course: Mutex::new(vec![]),
    });
    let response = new_course_handler(data, course).await;
    assert_eq!(response.status(), StatusCode::OK);
}
