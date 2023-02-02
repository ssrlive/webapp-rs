use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i32>,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl Course {
    pub fn new(teacher_id: i32, name: String) -> Self {
        Course {
            id: None,
            teacher_id,
            name,
            time: None,
        }
    }
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            id: course.id,
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}
