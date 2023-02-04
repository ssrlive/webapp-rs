use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Teacher {
    pub id: i64,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

impl Teacher {
    pub fn new(name: &str, picture_url: &str, profile: &str) -> Self {
        Teacher {
            id: -1,
            name: name.to_string(),
            picture_url: picture_url.to_string(),
            profile: profile.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

impl TryFrom<web::Json<CreateTeacher>> for CreateTeacher {
    type Error = ServiceError;

    fn try_from(value: web::Json<CreateTeacher>) -> Result<Self, Self::Error> {
        Ok(value.into_inner())
    }
}

impl From<Teacher> for CreateTeacher {
    fn from(teacher: Teacher) -> Self {
        CreateTeacher {
            name: teacher.name,
            picture_url: teacher.picture_url,
            profile: teacher.profile,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl TryFrom<web::Json<UpdateTeacher>> for UpdateTeacher {
    type Error = ServiceError;

    fn try_from(value: web::Json<UpdateTeacher>) -> Result<Self, Self::Error> {
        Ok(value.into_inner())
    }
}

impl From<Teacher> for UpdateTeacher {
    fn from(teacher: Teacher) -> Self {
        UpdateTeacher {
            name: Some(teacher.name),
            picture_url: Some(teacher.picture_url),
            profile: Some(teacher.profile),
        }
    }
}
