use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::fs;

use crate::models::Course;

const PG_MAX_CONN: u32 = 5;
const SQL_FILE: &str = "sql/courses.sql";

pub type Db = Pool<Postgres>;

async fn new_db_pool(db_url: &str, max_conn: u32) -> anyhow::Result<Db> {
    PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(db_url)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

async fn sql_exec(db: &Db, file: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(file)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    let sqls: Vec<&str> = content.split(';').collect();
    for sql in sqls {
        if sql.trim().is_empty() {
            continue;
        }
        sqlx::query(sql)
            .execute(db)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
    }
    Ok(())
}

pub async fn init_db(db_url: &str) -> anyhow::Result<Db> {
    {
        let db = new_db_pool(db_url, 1).await?;
        sql_exec(&db, SQL_FILE).await?;
    }
    let db = new_db_pool(db_url, PG_MAX_CONN).await?;
    Ok(db)
}

pub async fn get_courses_of_teacher_db(db: &Db, teacher_id: i64) -> anyhow::Result<Vec<Course>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, teacher_id, name, time
        FROM courses
        WHERE teacher_id = $1
        "#,
        teacher_id
    )
    .fetch_all(db)
    .await
    .map_err(|e| anyhow::anyhow!(e))?;

    rows.iter()
        .map(|row| {
            let mut course = Course::new(row.teacher_id, row.name.clone());
            course.set_id(row.id);
            course.set_time(row.time);
            Ok(course)
        })
        .collect()
}

pub async fn get_course_details_db(
    db: &Db,
    teacher_id: i64,
    course_id: i64,
) -> anyhow::Result<Course> {
    let row = sqlx::query!(
        r#"
        SELECT id, teacher_id, name, time
        FROM courses
        WHERE teacher_id = $1 AND id = $2
        "#,
        teacher_id,
        course_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| anyhow::anyhow!(e))?;

    let mut course = Course::new(row.teacher_id, row.name.clone());
    course.set_id(row.id);
    course.set_time(row.time);
    Ok(course)
}

pub async fn post_course_db(db: &Db, course: &Course) -> anyhow::Result<Course> {
    let row = sqlx::query!(
        r#"
        INSERT INTO courses (teacher_id, name)
        VALUES ($1, $2)
        RETURNING id, teacher_id, name, time
        "#,
        course.teacher_id,
        course.name
    )
    .fetch_one(db)
    .await
    .map_err(|e| anyhow::anyhow!(e))?;

    let mut course = Course::new(row.teacher_id, row.name.clone());
    course.set_id(row.id);
    course.set_time(row.time);
    Ok(course)
}