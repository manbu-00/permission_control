use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use DataAccessErr::*;

pub enum DataAccessErr {
    TypeErr,
    InvalidOperation,
}

fn slice_join<T: Display, S: Display + ?Sized>(
    f: &mut Formatter<'_>,
    v: &[T],
    separator: &S,
) -> FmtResult {
    if v.len() == 0 {
        return Ok(());
    }
    write!(f, "{}", v[0])?;
    let mut iter = v[1..].iter();
    iter.try_for_each(|x| write!(f, "{}{}", separator, x))
}

fn slice_join_bracket<T: Display, S: Display + ?Sized>(
    f: &mut Formatter<'_>,
    v: &[T],
    separator: &S,
) -> FmtResult {
    write!(f, "(")?;
    slice_join(f, v, separator)?;
    write!(f, ")")
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterNode {
    Logical(Logical, Vec<FilterNode>),
    Nullable(Nullable, Field),
    Equal(Eq, Field, Value),
    Cmp(Cmp, Field, Value),
    In(In, Field, ListValue),
    Like(Like, Field, StrValue),
}
impl FilterNode {
    pub fn check_operation(&self, info: &FieldInfo) -> Result<(), DataAccessErr> {
        let ope = match self {
            Self::Cmp(..) => " Cmp ",
            Self::Equal(..) => " Equal ",
            Self::In(..) => " In ",
            Self::Like(..) => " Like ",
            Self::Logical(..) => return Ok(()),
            Self::Nullable(..) => return info.nullable.then(|| ()).ok_or(InvalidOperation),
        };
        let has = info.operation.find(ope);
        has.and(Some(())).ok_or(InvalidOperation)
    }
}
impl Display for FilterNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Logical(ope, v) => slice_join_bracket(f, v, ope),
            Self::Nullable(ope, field) => write!(f, "({} {})", field, ope),
            Self::Cmp(ope, field, v) => write!(f, "{} {} {}", field, ope, v),
            Self::Equal(ope, field, v) => write!(f, "{} {} {}", field, ope, v),
            Self::In(ope, field, v) => write!(f, "{} {}{}", field, ope, v),
            Self::Like(ope, field, v) => match ope {
                Like::StartWith => write!(f, "{} LIKE '{}%'", field, v.escaped_like()),
                Like::Contains => write!(f, "{} LIKE '%{}%'", field, v.escaped_like()),
                Like::EndWith => write!(f, "{} LIKE '%{}'", field, v.escaped_like()),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Logical {
    And,
    Or,
}
impl Display for Logical {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Logical::And => write!(f, " AND "),
            Logical::Or => write!(f, " OR "),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Field(String);
impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize)]
pub enum Nullable {
    IsNull,
    IsNotNull,
}
impl Display for Nullable {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Nullable::IsNull => write!(f, "IS NULL"),
            Nullable::IsNotNull => write!(f, "IS NOT NULL"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Cmp {
    // Eq,
    // NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
}
impl Display for Cmp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            // Cmp::Eq => write!(f, "="),
            // Cmp::NotEq => write!(f, "<>"),
            Cmp::Gt => write!(f, ">"),
            Cmp::GtEq => write!(f, ">="),
            Cmp::Lt => write!(f, "<"),
            Cmp::LtEq => write!(f, "<="),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum In {
    In,
    NotIn,
}
impl Display for In {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            In::In => write!(f, "IN"),
            In::NotIn => write!(f, "NOT IN"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Eq {
    Eq,
    NotEq,
}
impl Display for Eq {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Eq::Eq => write!(f, "="),
            Eq::NotEq => write!(f, "<>"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Like {
    StartWith,
    Contains,
    EndWith,
}

#[derive(Serialize, Deserialize)]
pub struct StrValue(String);
impl StrValue {
    fn escaped(&self) -> String {
        // todo!()
        self.0.clone()
    }
    fn escaped_like(&self) -> String {
        todo!()
    }
}
impl Display for StrValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "'{}'", self.escaped())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Value {
    Id(u64), // Id 类型应该只能进行 eq, in 操作
    Int(u64),
    Num(f64),
    Str(StrValue),
    DateTime(StrValue),
    UnixTiemstamp(u64),
    CurrentUserId,
    CurrentDate,
    CurrentTime,
}
impl Value {
    pub fn chekc_type(&self, ftype: &FieldType) -> Result<(), DataAccessErr> {
        use FieldType as FT;
        match (self, ftype) {
            (Value::CurrentDate | Value::CurrentTime, FT::DateTime | FT::UnixTiemstamp)
            | (Value::Id(_) | Value::CurrentUserId, FT::Id)
            | (Value::Int(_), FieldType::Enum(_))
            | (Value::Str(_), FieldType::Str)
            | (Value::Int(_), FieldType::Int)
            | (Value::Num(_), FieldType::Num)
            | (Value::DateTime(_), FieldType::DateTime)
            | (Value::UnixTiemstamp(_), FieldType::UnixTiemstamp) => Ok(()),
            _ => Err(DataAccessErr::TypeErr),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Value::*;
        match self {
            Id(v) | Int(v) | UnixTiemstamp(v) => write!(f, "{}", v),
            Str(v) | DateTime(v) => write!(f, "{}", v),
            Num(v) => write!(f, "{}", v),
            CurrentUserId | CurrentDate | CurrentTime => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ListValue {
    Id(Vec<u64>),
    Int(Vec<u64>),
    Num(Vec<f64>),
    Str(Vec<StrValue>),
    DateTime(Vec<StrValue>),
    UnixTiemstamp(Vec<u64>),
}
impl ListValue {
    pub fn chekc_type(&self, ft: &FieldType) -> Result<(), DataAccessErr> {
        match (self, ft) {
            (ListValue::Str(_), FieldType::Enum(_))
            | (ListValue::Id(_), FieldType::Id)
            | (ListValue::Int(_), FieldType::Int)
            | (ListValue::Num(_), FieldType::Num)
            | (ListValue::Str(_), FieldType::Str)
            | (ListValue::DateTime(_), FieldType::DateTime)
            | (ListValue::UnixTiemstamp(_), FieldType::UnixTiemstamp) => Ok(()),
            _ => Err(DataAccessErr::TypeErr),
        }
    }
}

impl Display for ListValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use ListValue::*;
        match self {
            Int(v) | Id(v) | UnixTiemstamp(v) => slice_join_bracket(f, v, ", "),
            Str(v) | DateTime(v) => slice_join_bracket(f, v, ", "),
            Num(v) => slice_join_bracket(f, v, ", "),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FieldInfo {
    // pub list_key: String,
    pub name: String,
    pub text: String,
    pub nullable: bool,
    pub type_: FieldType,
    pub operation: String,
}

#[derive(Serialize, Deserialize)]
pub enum FieldType {
    Str,
    Int,
    Num,
    DateTime,
    UnixTiemstamp,
    Id, // Id 类型应该只能进行 eq 操作
    Enum(String),
}

use std::collections::HashMap;

pub fn field_info_map(infos: Vec<FieldInfo>) -> HashMap<String, FieldInfo> {
    infos.into_iter().map(|f| (f.name.clone(), f)).collect()
}

#[cfg(test)]
mod tests {
    use super::FilterNode::{self, *};
    use super::{Cmp::*, Eq::*, Field, Logical::*, StrValue, Value::*};
    // use super::{FieldInfo, In::*, FieldType as FT};
    #[test]
    fn serde_and_or() {
        #[rustfmt::skip]
        let root = &Logical(And,vec![
            Equal(Eq, Field("id".to_string()), Int(123)),
            Cmp(GtEq, Field("age".to_string()), Num(18.0)),
            Logical(Or, vec![
                Equal(Eq, Field("sex".to_string()),Str(StrValue("female".to_string()))),
                Logical(And, vec![
                    Equal(Eq, Field("sex".to_string()), Str(StrValue("male".to_string()))),
                    Cmp(LtEq, Field("age".to_string()), Num(30.0)),
                ]),
            ]),
        ]);
        // println!("{}", serde_json::to_string(root).unwrap());
        let res = "(id = 123 AND age >= 18 AND (sex = 'female' OR (sex = 'male' AND age <= 30)))";
        assert_eq!(root.to_string(), res);
    }

    // #[test]
    // fn check_type_and_or() {
    //     #[rustfmt::skip]
    //     let root = &Logical(And,vec![
    //         Equal(Eq, Field("id".to_string()), Int(123)),
    //         Cmp(GtEq, Field("age".to_string()), Num(18.0)),
    //         Logical(Or, vec![
    //             Equal(Eq, Field("sex".to_string()),Str(StrValue("female".to_string()))),
    //             Logical(And, vec![
    //                 Equal(Eq, Field("sex".to_string()), Str(StrValue("male".to_string()))),
    //                 Cmp(LtEq, Field("age".to_string()), Num(30.0)),
    //             ]),
    //         ]),
    //     ]);
    //     use FieldInfo as Fi;
    //     let field_infos = vec![Fi {
    //         name: "id".into(),
    //         text: "".into(),
    //         nullable: false,
    //         operation: "".into(),
    //         type_: FT::Id,
    //     }];
    // }

    #[test]
    fn serde_eq_node() {
        let json = r#"["Eq", "name", { "type": "Str", "value": "AAAA" }]"#;
        let eq: &FilterNode = &serde_json::from_str(json).unwrap();
        if let Equal(Eq, Field(field), Str(StrValue(value))) = eq {
            assert_eq!(field, "name");
            assert_eq!(value, "AAAA");
            // assert_eq!(serde_json::to_string(eq).unwrap(), json);
            return;
        };
        panic!("if let else, {}", eq);
    }

}

