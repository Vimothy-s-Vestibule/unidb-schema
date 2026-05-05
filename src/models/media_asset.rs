use ormlite::Model;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Media asset (e.g. profile pictures, discord attachments).
#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[ormlite(table = "media_assets")]
pub struct MediaAsset {
    #[ormlite(primary_key)]
    pub id: Uuid,

    /// HTML content-type
    pub content_type: String,

    /// S3 object key used in the Garage bucket
    pub object_key: String,

    /// Size of the asset in bytes (useful for caching constraints)
    pub size_bytes: Option<i64>,

    /// SHA-256 hash of the content for deduplication and cache integrity (ETag)
    pub content_hash: Option<String>,

    /// For similarity with other media
    #[serde(skip)]
    pub embedding: Option<Vector>,
}
