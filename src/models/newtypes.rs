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
pub enum ChannelType {
    #[default]
    Text,
    TextThread,
    ForumPost,
    Voice,
    Forum,
    Stage,
    Category,
}

impl ToSql<Text, Pg> for ChannelType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ChannelType::Text => out.write_all(b"text")?,
            ChannelType::Voice => out.write_all(b"voice")?,
            ChannelType::Forum => out.write_all(b"forum")?,
            ChannelType::Stage => out.write_all(b"stage")?,
            ChannelType::Category => out.write_all(b"category")?,
            ChannelType::TextThread => out.write_all(b"text_thread")?,
            ChannelType::ForumPost => out.write_all(b"forum_post")?,
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
            b"stage" => Ok(ChannelType::Stage),
            b"category" => Ok(ChannelType::Category),
            b"text_thread" => Ok(ChannelType::TextThread),
            b"forum_post" => Ok(ChannelType::ForumPost),

            _other => Err(format!("not a valid channel type: {:?}", _other).into()),
        }
    }
}

impl TryFrom<serenity::model::channel::ChannelType> for ChannelType {
    type Error = String;

    fn try_from(value: serenity::model::channel::ChannelType) -> Result<Self, Self::Error> {
        match value {
            serenity::model::channel::ChannelType::Text => Ok(ChannelType::Text),
            serenity::model::channel::ChannelType::Voice => Ok(ChannelType::Voice),
            serenity::model::channel::ChannelType::Forum => Ok(ChannelType::Forum),
            serenity::model::channel::ChannelType::Category => Ok(ChannelType::Category),
            serenity::model::channel::ChannelType::Stage => Ok(ChannelType::Stage),
            _other => Err(format!("not a base channel type: {:?}", _other)),
        }
    }
}
