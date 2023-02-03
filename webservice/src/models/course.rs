use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i64>,
    pub teacher_id: i64,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl Course {
    pub fn new(teacher_id: i64, name: String) -> Self {
        Course {
            id: None,
            teacher_id,
            name,
            time: None,
        }
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn set_time(&mut self, time: NaiveDateTime) {
        self.time = Some(time);
    }
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        course.into_inner()
    }
}
