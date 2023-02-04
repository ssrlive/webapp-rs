use super::dbinit::Db;
use crate::errors::{Result, ServiceError};
use crate::models::course::{Course, CreateCourse, UpdateCourse};

pub async fn get_courses_of_teacher_db(db: &Db, teacher_id: i64) -> Result<Vec<Course>> {
    let rows = sqlx::query_as!(
        Course,
        r#"
        SELECT * FROM course WHERE teacher_id = $1
        "#,
        teacher_id
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn get_course_details_db(db: &Db, teacher_id: i64, course_id: i64) -> Result<Course> {
    let row = sqlx::query_as!(
        Course,
        r#"
        SELECT * FROM course WHERE teacher_id = $1 AND id = $2
        "#,
        teacher_id,
        course_id
    )
    .fetch_optional(db)
    .await?;
    let error = format!("Course {course_id} of teacher {teacher_id} not found");
    row.ok_or(ServiceError::NotFound(error))
}

pub async fn post_course_db(db: &Db, course: CreateCourse) -> Result<Course> {
    let row = sqlx::query_as!(
        Course,
        r#"
        INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level
        "#,
        course.teacher_id,
        course.name,
        course.description,
        course.format,
        course.structure,
        course.duration,
        course.price,
        course.language,
        course.level
    )
    .fetch_one(db)
    .await?;

    Ok(row)
}

pub async fn update_course_db(
    db: &Db,
    teacher_id: i64,
    course_id: i64,
    course: UpdateCourse,
) -> Result<Course> {
    let new_course = course;
    let old_course = get_course_details_db(db, teacher_id, course_id).await?;

    let course = Course {
        id: course_id,
        teacher_id,
        name: new_course.name.unwrap_or(old_course.name),
        time: old_course.time,
        description: if new_course.description.is_some() {
            new_course.description
        } else {
            old_course.description
        },
        format: if new_course.format.is_some() {
            new_course.format
        } else {
            old_course.format
        },
        structure: if new_course.structure.is_some() {
            new_course.structure
        } else {
            old_course.structure
        },
        duration: if new_course.duration.is_some() {
            new_course.duration
        } else {
            old_course.duration
        },
        price: if new_course.price.is_some() {
            new_course.price
        } else {
            old_course.price
        },
        language: if new_course.language.is_some() {
            new_course.language
        } else {
            old_course.language
        },
        level: if new_course.level.is_some() {
            new_course.level
        } else {
            old_course.level
        },
    };

    let row = sqlx::query_as!(
        Course,
        r#"
        UPDATE course
        SET name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8
        WHERE teacher_id = $9 AND id = $10
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level
        "#,
        course.name,
        course.description,
        course.format,
        course.structure,
        course.duration,
        course.price,
        course.language,
        course.level,
        course.teacher_id,
        course.id
    )
    .fetch_optional(db)
    .await?;

    let error = format!("Course {course_id} of teacher {teacher_id} not found");
    row.ok_or(ServiceError::NotFound(error))
}

pub async fn delete_course_db(db: &Db, teacher_id: i64, course_id: i64) -> Result<Course> {
    let row = sqlx::query_as!(
        Course,
        r#"
        DELETE FROM course WHERE teacher_id = $1 and id = $2
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level
        "#,
        teacher_id,
        course_id
    )
    .fetch_optional(db)
    .await?;

    let error = format!("Course {course_id} of teacher {teacher_id} not found");
    row.ok_or(ServiceError::NotFound(error))
}
