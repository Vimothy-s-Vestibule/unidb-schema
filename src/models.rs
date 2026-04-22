//! Database models for unidb.
//!
//! # Module Organization
//!
//! - [`enums`] - All database TEXT enums
//! - [`user`] - VestibuleUser, DiscordAccount
//! - [`channel`] - Discord channels
//! - [`message`] - Discord messages
//! - [`reaction`] - Message reactions
//! - [`presence`] - Online status and rich presence activities
//! - [`score`] - HEXACO and behavioral traits
//! - [`skill`] - Skills and evidence
//! - [`connection`] - Connected third-party accounts
//! - [`activity`] - LLM-extracted activities
//! - [`media_asset`] - Media assets (profile pictures, etc)

pub mod activity;
pub mod channel;
pub mod connection;
pub mod enums;
pub mod job;
pub mod media_asset;
pub mod message;
pub mod presence;
pub mod reaction;
pub mod score;
pub mod skill;
pub mod user;
pub mod youtube;

pub use activity::Activity;
pub use channel::DiscordChannel;
pub use connection::{ConnectedAccount, ExternalContent, Platform};
pub use enums::*;
pub use job::Job;
pub use media_asset::MediaAsset;
pub use message::Message;
pub use presence::{ForcedOnlineEvidence, Presence};
pub use reaction::{DiscordEmoji, MessageReaction};
pub use score::{BehavioralTraits, HexacoTraits, Score};
pub use skill::{Skill, UserSkill, UserSkillEvidence};
pub use user::{DiscordAccount, VestibuleUser};
pub use youtube::{YoutubeComment, YoutubeVideo};
