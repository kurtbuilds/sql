use crate::util::SqlExtension;
use crate::{Dialect, Expr, ToSql};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenerationTime {
    Always,
    ByDefault,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenerationValue {
    Identity,
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Generated {
    pub time: GenerationTime,
    pub value: GenerationValue,
}

impl ToSql for Generated {
    fn write_sql(&self, buf: &mut String, dialect: Dialect) {
        buf.push_str("GENERATED ");
        match self.time {
            GenerationTime::Always => buf.push_str("ALWAYS "),
            GenerationTime::ByDefault => buf.push_str("BY DEFAULT "),
        }
        buf.push_str("AS ");
        match self.value {
            GenerationValue::Identity => buf.push_str("IDENTITY"),
            GenerationValue::Expr(ref expr) => {
                buf.push_sql(expr, dialect);
                buf.push_str(" STORED");
            }
        }
    }
}
