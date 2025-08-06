use sqlx::{Connection, PgConnection};
use sql::Schema;
use sql_sqlx::FromPostgres;

#[tokio::main]
async fn main() {
    let mut conn = PgConnection::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let schema = Schema::try_from_postgres(&mut conn, "public").await.unwrap();
    dbg!(schema);
}