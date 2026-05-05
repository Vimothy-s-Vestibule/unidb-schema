//! Rust models for unidb.
//!
//! - [`enums`] - All database TEXT enums
//! - [`user`] - VestibuleUser, DiscordAccount
//! - [`channel`] - Discord channels
//! - [`message`] - Discord messages
//! - [`reaction`] - Message reactions
//! - [`presence`] - Online status and rich presence activities
//! - [`score`] - HEXACO and other interesting traits (subject to experimentation)
//! - [`skill`] - Skills and evidence
//! - [`connection`] - Connected third-party accounts
//! - [`activity`] - LLM-extracted activities
//! - [`media_asset`] - Media assets (profile pictures, etc)

pub mod channel;
pub mod connection;
pub mod enums;
pub mod evidence;
pub mod job;
pub mod media_asset;
pub mod message;
pub mod message_attachments;
pub mod message_edits;
pub mod presence;
pub mod reaction;
pub mod score;
pub mod user;
pub mod user_facts_and_activities;
pub mod youtube;

pub use channel::DiscordChannel;
pub use connection::{ConnectedAccount, ExternalContent, Platform};
pub use enums::*;
pub use evidence::FactAndActivityEvidence;
pub use job::Job;
pub use media_asset::MediaAsset;
pub use message::Message;
pub use message_attachments::MessageAttachment;
pub use message_edits::MessageEdit;
pub use presence::Presence;
pub use reaction::{DiscordEmoji, MessageReaction};
pub use score::{BehavioralTraits, HexacoTraits, Score};
pub use user::{DiscordAccount, VestibuleUser};
pub use user_facts_and_activities::UserFactAndActivity;
pub use youtube::{YoutubeComment, YoutubeVideo};
