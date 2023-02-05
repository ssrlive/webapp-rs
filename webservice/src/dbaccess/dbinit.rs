const PG_MAX_CONN: usize = 5;
const SQL_FILE: &str = "sql/course.sql";

pub type Db = sqlx::Pool<sqlx::Postgres>;

async fn new_db_pool(db_url: &str, max_conn: usize) -> sqlx::Result<Db> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_conn as u32)
        .connect(db_url)
        .await
}

async fn sql_exec(db: &Db, file: &str) -> sqlx::Result<()> {
    let content = std::fs::read_to_string(file);
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

async fn _sql_exec(db: &Db, content: &str) -> sqlx::Result<()> {
    let sqls: Vec<&str> = content.split(';').collect();
    for sql in sqls {
        if sql.trim().is_empty() {
            continue;
        }
        sqlx::query(sql).execute(db).await?;
    }
    Ok(())
}

pub async fn db_initialize(db_url: &str, sql_file: Option<String>) -> sqlx::Result<Db> {
    {
        let db = new_db_pool(db_url, 1).await?;
        let sql_file = sql_file.unwrap_or_else(|| String::from(SQL_FILE));
        sql_exec(&db, &sql_file).await?;
    }
    let db = new_db_pool(db_url, PG_MAX_CONN).await?;
    Ok(db)
}
