use crate::errors::{Result, ServiceError};
use crate::models::course::{Course, CreateCourse};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::fs;

const PG_MAX_CONN: usize = 5;
const SQL_FILE: &str = "sql/course.sql";

pub type Db = Pool<Postgres>;

async fn new_db_pool(db_url: &str, max_conn: usize) -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(max_conn as u32)
        .connect(db_url)
        .await
        .map_err(ServiceError::from)
}

async fn sql_exec(db: &Db, file: &str) -> Result<()> {
    let content = fs::read_to_string(file).await?;
    let sqls: Vec<&str> = content.split(';').collect();
    for sql in sqls {
        if sql.trim().is_empty() {
            continue;
        }
        sqlx::query(sql).execute(db).await?;
    }
    Ok(())
}

pub async fn init_db(db_url: &str) -> Result<Db> {
    {
        let db = new_db_pool(db_url, 1).await?;
        sql_exec(&db, SQL_FILE).await?;
    }
    let db = new_db_pool(db_url, PG_MAX_CONN).await?;
    Ok(db)
}

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

pub async fn post_course_db(db: &Db, course: &CreateCourse) -> Result<Course> {
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
