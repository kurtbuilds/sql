/// Defines structs and functions for auto-generating migrations.
pub mod migrate;
/// Defines structs and functions for representing SQL queries.
pub mod query;
/// Defines structs and functions for representing SQL database schemas.
pub mod schema;

mod to_sql;
pub mod util;

#[doc(inline)]
pub use migrate::{Migration, MigrationOptions, migrate};
#[doc(inline)]
pub use query::{
    AlterTable, CreateIndex, CreateTable, Cte, CteQuery, Expr, From, IndexType, Insert, Operation,
    OrderBy, Select, SelectColumn, Union, Where,
};
#[doc(inline)]
pub use schema::{Column, Constraint, Index, Schema, Table, Type};
#[doc(inline)]
pub use to_sql::{Dialect, ToSql};
