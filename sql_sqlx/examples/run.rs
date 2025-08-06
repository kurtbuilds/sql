use sql::Schema;
use sql_sqlx::{FromPostgres, query_functions};
use sqlx::{Connection, PgConnection};

#[tokio::main]
async fn main() {
    let mut conn = PgConnection::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let schema = Schema::try_from_postgres(&mut conn, "public")
        .await
        .unwrap();
    let functions = query_functions(&mut conn, "public").await.unwrap();
    dbg!(functions);
}
