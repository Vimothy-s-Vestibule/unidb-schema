//! Database models for UniDB.
//!
//! # Module Organization
//!
//! - [`enums`] - All database TEXT enums
//! - [`user`] - VestibuleUser, DiscordAccount
//! - [`channel`] - Discord channels
//! - [`message`] - Discord messages
//! - [`reaction`] - Message reactions
//! - [`presence`] - Online status and activities
//! - [`score`] - HEXACO and behavioral traits
//! - [`skill`] - Skills and evidence
//! - [`platform`] - External platform integrations
//! - [`activity`] - LLM-extracted activities

pub mod activity;
pub mod channel;
pub mod enums;
pub mod message;
pub mod platform;
pub mod presence;
pub mod reaction;
pub mod score;
pub mod skill;
pub mod user;

// Re-export all types at module root for convenience
pub use activity::Activity;
pub use channel::Channel;
pub use enums::*;
pub use message::Message;
pub use platform::{ExternalContent, Platform, UserPlatformLink};
pub use presence::{ForcedOnlineEvidence, Presence, PresenceActivity};
pub use reaction::Reaction;
pub use score::{BehavioralTraits, HexacoTraits, Score};
pub use skill::{Skill, UserSkill, UserSkillEvidence};
pub use user::{DiscordAccount, VestibuleUser};
