use anyhow::{Error, Result};
use itertools::Itertools;
use sql::{Column, Expr, Generated, GenerationTime, GenerationValue, Schema, Table, schema};
use sqlx::PgConnection;
use std::str::FromStr;

#[allow(async_fn_in_trait)]
pub trait FromPostgres: Sized {
    async fn try_from_postgres(conn: &mut PgConnection, schema_name: &str) -> Result<Self>;
}

#[derive(sqlx::FromRow)]
pub struct ColumnRow {
    pub table_name: String,
    pub column_name: String,
    #[allow(dead_code)]
    pub ordinal_position: i32,
    pub is_nullable: String,
    pub data_type: String,
    pub numeric_precision: Option<i32>,
    pub numeric_scale: Option<i32>,
    pub inner_type: Option<String>,
    pub primary_key: bool,
    pub generation_time: Option<String>,
    pub generation_expression: Option<String>,
    pub identity_generation: Option<String>,
}

pub async fn query_schema_columns(
    conn: &mut PgConnection,
    schema_name: &str,
) -> Result<Vec<ColumnRow>> {
    let s = include_str!("sql/query_columns.sql");
    let result = sqlx::query_as::<_, ColumnRow>(s)
        .bind(schema_name)
        .fetch_all(conn)
        .await?;
    Ok(result)
}

#[derive(sqlx::FromRow)]
struct TableSchema {
    #[allow(dead_code)]
    pub table_schema: String,
    pub table_name: String,
}

pub async fn query_table_names(conn: &mut PgConnection, schema_name: &str) -> Result<Vec<String>> {
    let s = include_str!("sql/query_tables.sql");
    let result = sqlx::query_as::<_, TableSchema>(s)
        .bind(schema_name)
        .fetch_all(conn)
        .await?;
    Ok(result.into_iter().map(|t| t.table_name).collect())
}

#[derive(Debug, sqlx::FromRow)]
pub struct ForeignKey {
    pub table_schema: String,
    pub constraint_name: String,
    pub table_name: String,
    pub column_name: String,
    pub foreign_table_schema: String,
    pub foreign_table_name: String,
    pub foreign_column_name: String,
}

pub async fn query_constraints(
    conn: &mut PgConnection,
    schema_name: &str,
) -> Result<Vec<ForeignKey>> {
    let s = include_str!("sql/query_constraints.sql");
    Ok(sqlx::query_as::<_, ForeignKey>(s)
        .bind(schema_name)
        .fetch_all(conn)
        .await?)
}

#[derive(Debug, sqlx::FromRow)]
pub struct Index {
    pub schema: String,
    pub table: String,
    pub name: String,
    // text of the index definition
    pub statement: String,
    pub unique: bool,
    pub kind: String,
    pub columns: Vec<String>,
}

pub async fn query_indices(conn: &mut PgConnection, schema_name: &str) -> Result<Vec<Index>> {
    // because of pg_tables join, this only returns indices for tables, not views/mat views
    let s = include_str!("sql/query_indices.sql");
    Ok(sqlx::query_as::<_, Index>(s)
        .bind(schema_name)
        .fetch_all(conn)
        .await?)
}

#[derive(Debug, sqlx::FromRow)]
pub struct Function {
    pub routine_schema: String,
    pub routine_name: String,
    pub routine_type: String,
    pub data_type: Option<String>,
    pub routine_definition: Option<String>,
}

pub async fn query_functions(conn: &mut PgConnection, schema_name: &str) -> Result<Vec<Function>> {
    let s = include_str!("sql/query_functions.sql");
    Ok(sqlx::query_as::<_, Function>(s)
        .bind(schema_name)
        .fetch_all(conn)
        .await?)
}

#[derive(sqlx::FromRow)]
pub struct Trigger {
    pub trigger_schema: String,
    pub trigger_name: String,
    pub event_manipulation: String,
    pub event_object_table: String,
    pub action_timing: String,
    pub action_statement: String,
}

pub async fn query_triggers(conn: &mut PgConnection, schema_name: &str) -> Result<Vec<Trigger>> {
    let s = include_str!("sql/query_triggers.sql");
    Ok(sqlx::query_as::<_, Trigger>(s)
        .bind(schema_name)
        .fetch_all(conn)
        .await?)
}

fn parse_generated(
    time: Option<String>,
    expr: Option<String>,
    identity: Option<String>,
) -> Option<Generated> {
    let time = time?;
    let time = match time.as_str() {
        "ALWAYS" => GenerationTime::Always,
        "BY DEFAULT" => GenerationTime::ByDefault,
        _ => return None,
    };
    let value = if let Some(identity) = identity
        && identity == "YES"
    {
        GenerationValue::Identity
    } else if let Some(expr) = expr {
        GenerationValue::Expr(Expr::Raw(expr))
    } else {
        return None;
    };
    Some(Generated { time, value })
}

impl TryInto<Column> for ColumnRow {
    type Error = Error;

    fn try_into(self) -> std::result::Result<Column, Self::Error> {
        use schema::Type::*;
        let nullable = self.is_nullable == "YES";
        let typ = match self.data_type.as_str() {
            "ARRAY" => {
                let inner = schema::Type::from_str(
                    &self
                        .inner_type
                        .expect("Encounterd ARRAY with no inner type."),
                )?;
                Array(Box::new(inner))
            }
            "numeric" if self.numeric_precision.is_some() && self.numeric_scale.is_some() => {
                Numeric(
                    self.numeric_precision.unwrap() as u8,
                    self.numeric_scale.unwrap() as u8,
                )
            }
            z => schema::Type::from_str(z)?,
        };
        let generated = parse_generated(
            self.generation_time,
            self.generation_expression,
            self.identity_generation,
        );
        Ok(Column {
            name: self.column_name.clone(),
            typ,
            nullable,
            primary_key: false,
            default: None,
            constraint: None,
            generated,
        })
    }
}

impl FromPostgres for Schema {
    async fn try_from_postgres(conn: &mut PgConnection, schema: &str) -> Result<Schema> {
        let column_schemas = query_schema_columns(conn, schema).await?;
        let mut tables = column_schemas
            .into_iter()
            .chunk_by(|c| c.table_name.clone())
            .into_iter()
            .map(|(table_name, group)| {
                let columns = group
                    .map(|c: ColumnRow| c.try_into())
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(Table {
                    schema: Some(schema.to_string()),
                    name: table_name,
                    columns,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let constraints = query_constraints(conn, schema).await?;
        let mut it_tables = tables.iter_mut().peekable();
        for fk in constraints {
            while &fk.table_name != &it_tables.peek().unwrap().name {
                it_tables.next();
            }
            let table = it_tables.peek_mut().unwrap();
            let column = table
                .columns
                .iter_mut()
                .find(|c| c.name == fk.column_name)
                .expect("Constraint for unknown column.");
            column.constraint = Some(schema::Constraint::ForeignKey(schema::ForeignKey {
                table: fk.foreign_table_name,
                columns: vec![fk.foreign_column_name],
            }));
        }

        // Degenerate case but you can have tables with no columns...
        let table_names = query_table_names(conn, schema).await?;
        let mut tables_it = tables.iter().peekable();
        let mut empty_tables = Vec::new();
        'outer: for name in table_names {
            while let Some(table) = tables_it.peek() {
                if &name == &table.name {
                    tables_it.next();
                    continue 'outer;
                }
            }
            empty_tables.push(Table {
                schema: Some(schema.to_string()),
                name,
                columns: vec![],
            })
        }
        Ok(Schema { tables })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_numeric() {
        let c = ColumnRow {
            table_name: "foo".to_string(),
            column_name: "bar".to_string(),
            ordinal_position: 1,
            is_nullable: "NO".to_string(),
            data_type: "numeric".to_string(),
            numeric_precision: Some(10),
            numeric_scale: Some(2),
            inner_type: None,
            primary_key: false,
        };
        let column: Column = c.try_into().unwrap();
        assert_eq!(column.typ, schema::Type::Numeric(10, 2));
    }

    #[test]
    fn test_integer() {
        let c = ColumnRow {
            table_name: "foo".to_string(),
            column_name: "bar".to_string(),
            ordinal_position: 1,
            is_nullable: "NO".to_string(),
            data_type: "integer".to_string(),
            numeric_precision: Some(32),
            numeric_scale: Some(0),
            inner_type: None,
            primary_key: false,
        };
        let column: Column = c.try_into().unwrap();
        assert_eq!(column.typ, schema::Type::I32);
    }
}
