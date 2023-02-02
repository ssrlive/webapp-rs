use crate::models::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

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
    let count = state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.teacher_id == course.teacher_id)
        .collect::<Vec<_>>()
        .len();
    let new_course = Course {
        id: Some(count + 1),
        teacher_id: course.teacher_id,
        name: course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    state.courses.lock().unwrap().push(new_course.clone());
    HttpResponse::Ok().json(&new_course)
}

pub async fn get_courses_of_teacher(
    state: web::Data<AppState>,
    params: web::Path<usize>,
) -> HttpResponse {
    let teacher_id = params.into_inner();
    let courses = state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.teacher_id == teacher_id)
        .collect::<Vec<_>>();
    if !courses.is_empty() {
        HttpResponse::Ok().json(&courses)
    } else {
        HttpResponse::Ok().json("No courses found")
    }
}

pub async fn get_course_detail(
    state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (teacher_id, course_id) = params.into_inner();
    let course = state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.teacher_id == teacher_id && c.id == Some(course_id))
        .collect::<Vec<_>>();
    if !course.is_empty() {
        HttpResponse::Ok().json(&course)
    } else {
        HttpResponse::Ok().json("No course found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course::new(1, "Math".to_string()));
        let state = AppState::new("I'm healthy");
        let state = web::Data::new(state);
        let response = new_course_handler(state, course).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_of_teacher() {
        let state = AppState::new("I'm healthy");
        let course = Course::new(1, "Math".to_string());
        state.courses.lock().unwrap().push(course);
        let state = web::Data::new(state);
        let teacher_id = web::Path::from(1);
        let response = get_courses_of_teacher(state, teacher_id).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        let state = AppState::new("I'm healthy");
        let course = Course::new(1, "Math".to_string());
        state.courses.lock().unwrap().push(course);
        let state = web::Data::new(state);
        let params = web::Path::from((1, 1));
        let response = get_course_detail(state, params).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
