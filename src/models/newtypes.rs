use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sql_types::{Array, Nullable};
use diesel::AsExpression;

use serde::{Deserialize, Serialize};

use std::io::Write;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Array<Nullable<Text>>)]
#[serde(transparent)]
pub struct TextVec(pub Vec<String>);

impl FromSql<Array<Nullable<Text>>, Pg> for TextVec {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let vec: Vec<Option<String>> = FromSql::<Array<Nullable<Text>>, Pg>::from_sql(bytes)?;
        let strings = vec.into_iter().flatten().collect();
        Ok(TextVec(strings))
    }
}

impl ToSql<Array<Nullable<Text>>, Pg> for TextVec {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<Array<Text>, Pg>::to_sql(&self.0, out)
    }
}

impl From<Vec<String>> for TextVec {
    fn from(vec: Vec<String>) -> Self {
        Self(vec)
    }
}

impl From<TextVec> for Vec<String> {
    fn from(tv: TextVec) -> Self {
        tv.0
    }
}

impl std::ops::Deref for TextVec {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TextVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// -------------------------

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow, Default,
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "lowercase")]
pub enum RecordStatus {
    #[default]
    Pending,
    Scored,
}

impl ToSql<Text, Pg> for RecordStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RecordStatus::Pending => out.write_all(b"pending")?,
            RecordStatus::Scored => out.write_all(b"scored")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for RecordStatus {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(RecordStatus::Pending),
            b"scored" => Ok(RecordStatus::Scored),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// -------------------------

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow, Default,
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    #[default]
    Text,
    Voice,
    Forum,
    Thread,
    Stage,
}

impl ToSql<Text, Pg> for ChannelType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ChannelType::Text => out.write_all(b"text")?,
            ChannelType::Voice => out.write_all(b"voice")?,
            ChannelType::Forum => out.write_all(b"forum")?,
            ChannelType::Thread => out.write_all(b"thread")?,
            ChannelType::Stage => out.write_all(b"stage")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for ChannelType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"text" => Ok(ChannelType::Text),
            b"voice" => Ok(ChannelType::Voice),
            b"forum" => Ok(ChannelType::Forum),
            b"thread" => Ok(ChannelType::Thread),
            b"stage" => Ok(ChannelType::Stage),

            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
