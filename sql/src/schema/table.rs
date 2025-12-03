use crate::{Dialect, ToSql};
use crate::schema::column::Column;
use crate::util::SqlExtension;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Table {
    pub schema: Option<String>,
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn primary_key(&self) -> Option<&Column> {
        self.columns.iter().find(|c| c.primary_key)
    }

    pub fn new(name: &str) -> Table {
        Table {
            schema: None,
            name: name.to_string(),
            columns: vec![],
        }
    }

    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    pub fn schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }
}

impl ToSql for Table {
    fn write_sql(&self, buf: &mut String, dialect: Dialect) {
        buf.push_str("CREATE TABLE IF NOT EXISTS ");
        buf.push_table_name(&self.schema, &self.name);
        buf.push_str(" (\n");
        buf.push_sql_sequence(&self.columns, ",\n", dialect);
        buf.push_str("\n)");
    }
}
