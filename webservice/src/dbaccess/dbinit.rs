use crate::errors::{Result, ServiceError};
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
    let content = fs::read_to_string(file).await;
    match content {
        Ok(content) => {
            _sql_exec(db, &content).await?;
        }
        Err(e) => {
            println!("read sql file error: {e}");
        }
    }
    Ok(())
}

async fn _sql_exec(db: &Db, content: &str) -> Result<()> {
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
