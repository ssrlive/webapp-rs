# webapp-rs

Just a demo web app powered by actix, sqlx and tera.

Since sqlx needs the `DATABASE_URL` environment variable to be compiled,
please prepare the [PostgreSQL](https://www.postgresql.org/) database server before compiling this project,
and prepare the `.env` file in the current working directory, and import the data table from the [sql file](webservice/sql/course.sql)
in the `psql` terminal.

In this project the content of the `.env` file is as follows.

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
SQL_FILE_PATH=webservice/sql/course.sql
BACKEND_HOST_PORT=127.0.0.1:3000

FRONTEND_HOST_PORT=127.0.0.1:8080
FRONTEND_TO_BACKEND_URL=http://127.0.0.1:3000
```
