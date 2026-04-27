//! Media asset models.

use pgvector::Vector;
use serde::{Deserialize, Serialize};
use ormlite::Model;
use uuid::Uuid;

/// Media asset (e.g. profile pictures).
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: Uuid,

    /// HTML content-type
    pub content_type: String,

    /// The actual content
    pub bytes: String,

    /// For similarity with other media
    #[serde(skip)]
    pub embedding: Option<Vector>,
}
