//! Message reaction model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

/// Idiomatic Rust representation of a Discord Emoji.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Emoji {
    /// Standard unicode emoji (e.g., "👍").
    Unicode(String),
    /// Custom Discord emoji.
    Custom {
        id: String,
        name: Option<String>,
        is_animated: bool,
        url: Option<String>,
    },
}

/// Emoji reaction on a Discord message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub id: Uuid,
    pub message_id: i64,
    pub user_id: i64,

    /// The emoji used for the reaction.
    pub emoji: Emoji,

    pub reacted_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Reaction {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let is_custom: bool = row.try_get("is_custom")?;

        let emoji = if is_custom {
            Emoji::Custom {
                id: row.try_get("emoji")?,
                name: row.try_get("emoji_name")?,
                is_animated: row.try_get("is_animated")?,
                url: row.try_get("emoji_url")?,
            }
        } else {
            Emoji::Unicode(row.try_get("emoji")?)
        };

        Ok(Self {
            id: row.try_get("id")?,
            message_id: row.try_get("message_id")?,
            user_id: row.try_get("user_id")?,
            emoji,
            reacted_at: row.try_get("reacted_at")?,
        })
    }
}
