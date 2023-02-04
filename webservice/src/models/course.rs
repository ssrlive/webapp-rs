use crate::errors::ServiceError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Course {
    pub id: i64,
    pub teacher_id: i64,
    pub name: String,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl Course {
    pub fn new(teacher_id: i64, name: &str) -> Self {
        Course {
            id: -1,
            teacher_id,
            name: name.to_string(),
            time: None,
            description: None,
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: None,
            level: None,
        }
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn set_time(&mut self, time: NaiveDateTime) {
        self.time = Some(time);
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateCourse {
    pub teacher_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<Course> for CreateCourse {
    fn from(course: Course) -> Self {
        CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name,
            description: course.description,
            format: course.format,
            structure: course.structure,
            duration: course.duration,
            price: course.price,
            language: course.language,
            level: course.level,
        }
    }
}

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = ServiceError;

    fn try_from(json: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(json.into_inner())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<Course> for UpdateCourse {
    fn from(course: Course) -> Self {
        UpdateCourse {
            name: Some(course.name),
            description: course.description,
            format: course.format,
            structure: course.structure,
            duration: course.duration,
            price: course.price,
            language: course.language,
            level: course.level,
        }
    }
}

impl TryFrom<web::Json<UpdateCourse>> for UpdateCourse {
    type Error = ServiceError;

    fn try_from(json: web::Json<UpdateCourse>) -> Result<Self, Self::Error> {
        Ok(json.into_inner())
    }
}
