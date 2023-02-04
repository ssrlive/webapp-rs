use crate::dbaccess::dbinit::Db;
use crate::errors::{Result, ServiceError};
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};

pub async fn get_teachers_db(db: &Db) -> Result<Vec<Teacher>> {
    sqlx::query_as!(
        Teacher,
        r#"
        SELECT * FROM teacher
        "#
    )
    .fetch_all(db)
    .await
    .map_or_else(|e| Err(e.into()), Ok)
}

pub async fn get_teacher_db(db: &Db, teacher_id: i64) -> Result<Teacher> {
    let error = ServiceError::NotFound(format!("Teacher {teacher_id} not found"));
    sqlx::query_as!(
        Teacher,
        r#"
        SELECT * FROM teacher WHERE id = $1
        "#,
        teacher_id
    )
    .fetch_optional(db)
    .await
    .map_or_else(|e| Err(e.into()), |o| o.ok_or(error))
}

pub async fn create_teacher_db(db: &Db, teacher: CreateTeacher) -> Result<Teacher> {
    sqlx::query_as!(
        Teacher,
        r#"
        INSERT INTO teacher (name, picture_url, profile)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        teacher.name,
        teacher.picture_url,
        teacher.profile
    )
    .fetch_one(db)
    .await
    .map_or_else(|e| Err(e.into()), Ok)
}

pub async fn update_teacher_db(
    db: &Db,
    teacher_id: i64,
    teacher: UpdateTeacher,
) -> Result<Teacher> {
    let new_teacher = teacher;
    let old_teacher = get_teacher_db(db, teacher_id).await?;
    let teacher = Teacher {
        id: old_teacher.id,
        name: if let Some(name) = new_teacher.name {
            name
        } else {
            old_teacher.name
        },
        picture_url: if let Some(picture_url) = new_teacher.picture_url {
            picture_url
        } else {
            old_teacher.picture_url
        },
        profile: if let Some(profile) = new_teacher.profile {
            profile
        } else {
            old_teacher.profile
        },
    };

    sqlx::query_as!(
        Teacher,
        r#"
        UPDATE teacher
        SET name = $1, picture_url = $2, profile = $3
        WHERE id = $4
        RETURNING *
        "#,
        teacher.name,
        teacher.picture_url,
        teacher.profile,
        teacher.id
    )
    .fetch_one(db)
    .await
    .map_or_else(|e| Err(e.into()), Ok)
}

pub async fn delete_teacher_db(db: &Db, teacher_id: i64) -> Result<Teacher> {
    sqlx::query_as!(
        Teacher,
        r#"
        DELETE FROM teacher
        WHERE id = $1
        RETURNING *
        "#,
        teacher_id
    )
    .fetch_one(db)
    .await
    .map_or_else(|e| Err(e.into()), Ok)
}
