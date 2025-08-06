<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/sql/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/sql.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/sql/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/sql.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/sql/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/kurtbuilds/sql/test.yaml?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/sql">
    <img src="https://img.shields.io/crates/d/sql?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/sql">
    <img src="https://img.shields.io/crates/v/sql?style=flat-square" alt="Crates.io" />
</a>

</p>

# `sql`
`sql` is a set of primitives to represent SQL tables and queries. Use these primitives to:
- **Auto-generate migrations**: Load SQL representations in a standardized form (`sql::Schema`), calculate differences between 
schemas (`sql::Migration`), and generate SQL to apply the migration (`sql::Migration::to_sql`).
- **Build SQL queries**: Represent SQL queries in a data model, to create APIs for query generation. Then, generate the
SQL query. *Note: this library does not support parsing SQL queries (yet).*

For auto-generating migrations, there are a few built-in schema sources:
- **Postgres**: [`sql_sqlx`](./sql_sqlx)
- **OpenAPI v3**: [`sql_openapi`](./sql_openapi)

If you need another source, you should define a way to build a `sql::Schema` from your data source, then use `sql` 
to auto-generate migrations.

Current tools that support this:

- [`ormlite`](https://github.com/kurtbuilds/ormlite)

If you use this library, submit a PR to be added to this list.

## Usage

This example reads the schema from a postgres database, defines an empty schema (which should be filled in),
and then computes the migration to apply to the database.

```rust
use sql_sqlx::FromPostgres;

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL").unwrap();
    let mut conn = sqlx::postgres::PgConnection::connect(&url).await?;
    let current = Schema::try_from_postgres(&mut conn, schema_name).await?;
    let end_state = Schema::default(); // Load your end-state by manually defining it, or building it from another source
    let migration = current.migrate_to(end_state, &sql::Options::default());
    
    for statement in migration.statements {
        let statement = statement.to_sql(Dialect::Postgres);
        println!("{}", statement);
    }
}
```

# Roadmap

- [ ] When calculating migrations, create commented out lines for column deletion
- [ ] ? When calculating migrations, do alter column by calculating word distance between column names
