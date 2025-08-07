/// Defines structs and functions for auto-generating migrations.
pub mod migrate;
/// Defines structs and functions for representing SQL queries.
pub mod query;
/// Defines structs and functions for representing SQL database schemas.
pub mod schema;

mod to_sql;
pub mod util;

#[doc(inline)]
pub use migrate::{migrate, Migration, MigrationOptions};
#[doc(inline)]
pub use query::*;
#[doc(inline)]
pub use schema::*;
#[doc(inline)]
pub use to_sql::{Dialect, ToSql};
