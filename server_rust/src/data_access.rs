use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

fn slice_join<T: Display>(f: &mut Formatter<'_>, v: &[T], separator: &str) -> FmtResult {
    if v.len() == 0 {
        return Ok(());
    }
    write!(f, "{}", v[0])?;
    let mut iter = v.iter().skip(1);
    iter.try_for_each(|x| write!(f, "{}{}", separator, x))
}

fn slice_join_bracket<T: Display>(f: &mut Formatter<'_>, v: &[T], separator: &str) -> FmtResult {
    write!(f, "(")?;
    slice_join(f, v, separator)?;
    write!(f, ")")
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LogicalNode {
    And(Vec<LogicalNode>),
    Or(Vec<LogicalNode>),
    // Cmp(CmpNode),
    IsNull(Field),
    IsNotNull(Field),
    Eq(Field, ValueNode),
    NotEq(Field, ValueNode),
    Gt(Field, ValueNode),
    Gte(Field, ValueNode),
    Lt(Field, ValueNode),
    Lte(Field, ValueNode),
    In(Field, ListNode),
    NotIn(Field, ListNode),
    // Not(Box<LogicalNode>), 可以不提供
}
impl Display for LogicalNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LogicalNode::*;
        match self {
            And(v) => slice_join_bracket(f, v, " AND "),
            Or(v) => slice_join_bracket(f, v, " OR "),
            IsNull(v) => write!(f, "({} IS NULL)", v),
            IsNotNull(v) => write!(f, "({} IS NOT NULL)", v),
            Eq(l, r) => write!(f, "({} = {})", l, r),
            NotEq(l, r) => write!(f, "({} <> {})", l, r),
            Gt(l, r) => write!(f, "({} > {})", l, r),
            Gte(l, r) => write!(f, "({} >= {})", l, r),
            Lt(l, r) => write!(f, "({} < {})", l, r),
            Lte(l, r) => write!(f, "({} <= {})", l, r),
            In(l, r) => write!(f, "({} IN{})", l, r),
            NotIn(l, r) => write!(f, "({} NOT IN{})", l, r),
            // _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueNode {
    Str(String),
    Int(i64),
    Num(f64),
    DateTime(String),
    UnixTimestamp(u64),
    // Field(String),
    // Null, // 应该使用 IsNull, IsNotNull
    CurrentUserId,
    CurrentTime,
    CurrentDate,
}

impl Display for ValueNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ValueNode::*;
        match self {
            Str(v) | DateTime(v) => write!(f, "'{}'", v),
            Int(v) => write!(f, "{}", v),
            Num(v) => write!(f, "{}", v),
            // Field(v) => write!(f, "[{}]", v),
            UnixTimestamp(v) => write!(f, "{}", v),
            CurrentUserId => write!(f, "@CurrentUserId"),
            _ => todo!(), // 由数据库 sql 确定
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field(String);

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self)
    }
}

pub struct TblFilterConfig {
    pub field: String,
    // pub type
}

pub struct FieldInfo {
    pub name: String,
    pub type_: FieldType,
}

// /// field_type
// mod ft {
pub enum FieldType {
    Int(Int),
    Str(Str),
    // DateTime(),
}
pub trait FieldType_ {}
impl FieldType_ for () {}
pub struct Int;
impl FieldType_ for Int {}
pub struct Str;
impl FieldType_ for Str {}

// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListNode {
    StrList(Vec<String>),
    IntList(Vec<i64>),
    NumList(Vec<f64>),
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ListNode::*;
        match self {
            StrList(v) => {
                if v.len() == 0 {
                    write!(f, "()")
                } else {
                    write!(f, "('")?;
                    slice_join(f, v, "', '")?;
                    write!(f, "')")
                }
            }
            IntList(v) => slice_join_bracket(f, v, ", "),
            NumList(v) => slice_join_bracket(f, v, ", "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Field, ListNode::*, LogicalNode::*, ValueNode::*};
    #[test]
    fn and_or() {
        let root = &And(vec![
            Eq(Field("id".to_string()), Int(123)),
            Gte(Field("age".to_string()), Num(18.0)),
            Or(vec![
                Eq(Field("sex".to_string()), Str("female".to_string())),
                And(vec![
                    Eq(Field("sex".to_string()), Str("male".to_string())),
                    Lte(Field("age".to_string()), Num(30.0)),
                ]),
            ]),
        ]);
        assert_eq!(root.to_string(), "(([id] = 123) AND ([age] >= 18) AND (([sex] = 'female') OR (([sex] = 'male') AND ([age] <= 30))))");
        println!("{}", serde_json::to_string(root).unwrap());
    }

    #[test]
    fn single() {
        let root = Eq(Field("id".to_string()), Str("ABC".to_string()));
        assert_eq!(root.to_string(), "([id] = 'ABC')");
    }

    #[test]
    fn in_list() {
        let root = In(Field("id".to_string()), IntList(vec![1, 2, 3]));
        assert_eq!(root.to_string(), "([id] IN(1, 2, 3))");
    }

    #[test]
    fn int_str_list() {
        let root = In(
            Field("id".to_string()),
            StrList(vec!["a".to_string(), "b".to_string()]),
        );
        assert_eq!(root.to_string(), "([id] IN('a', 'b'))");
    }

    #[test]
    fn not_in_ist() {
        let root = NotIn(Field("id".to_string()), IntList(vec![1, 2, 3]));
        assert_eq!(root.to_string(), "([id] NOT IN(1, 2, 3))");
    }

    #[test]
    fn value_node_literal() {
        let root = &Eq(Field("time".to_string()), CurrentTime);
        println!("{}", serde_json::to_string(root).unwrap());
    }
}

