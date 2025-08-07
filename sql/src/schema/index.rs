use crate::{Dialect, ToSql};
use crate::util::SqlExtension;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexKind {
    BTree,
    Hash,
    Gist,
    SpGist,
    Brin,
    Other(String),
}

impl Default for IndexKind {
    fn default() -> Self {
        IndexKind::BTree
    }
}

/// Create index action for a table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    pub name: String,
    pub unique: bool,
    pub schema: Option<String>,
    pub table: String,
    pub columns: Vec<String>,
    pub kind: IndexKind,
}

impl ToSql for Index {
    fn write_sql(&self, buf: &mut String, _: Dialect) {
        buf.push_str("CREATE ");
        if self.unique {
            buf.push_str("UNIQUE ");
        }
        buf.push_quoted(&self.name);
        buf.push_str(" ON ");
        buf.push_table_name(&self.schema, &self.table);
        buf.push_str(" USING ");
        match &self.kind {
            // btree is default
            IndexKind::BTree => {}
            IndexKind::Hash => buf.push_str("HASH"),
            IndexKind::Gist => buf.push_str("GIST"),
            IndexKind::SpGist => buf.push_str("SPGIST"),
            IndexKind::Brin => buf.push_str("BRIN"),
            IndexKind::Other(kind) => buf.push_str(kind),
        }
        buf.push_str(" (");
        buf.push_quoted_sequence(&self.columns, ", ");
        buf.push(')');
    }
}