-- ============================================================================
-- UniDB Schema
-- Run against a blank PostgreSQL database to initialize all tables
-- Tables are ordered to satisfy foreign key dependencies
-- ============================================================================

-- Load vector extension
CREATE EXTENSION IF NOT EXISTS "vector";

-- Default to public schema
SET search_path TO public;

-- ============================================================================
-- INDEPENDENT TABLES (no foreign keys)
-- ============================================================================

CREATE TABLE scores (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  -- HEXACO
  honesty double precision NOT NULL,
  emotionality double precision NOT NULL,
  extraversion double precision NOT NULL,
  agreeableness double precision NOT NULL,
  conscientiousness double precision NOT NULL,
  openness_to_experience double precision NOT NULL,

 -- Other interesting traits
  agency double precision NOT NULL,
  achievement double precision NOT NULL,
  influence double precision NOT NULL,
  sarcasm double precision NOT NULL,
  security double precision NOT NULL,
  self_reflection double precision NOT NULL,
  technical_competence double precision NOT NULL,
  busyness double precision NOT NULL,

  embedding vector
);

CREATE TABLE channels (
  channel_id bigint PRIMARY KEY,
  name text NOT NULL,
  -- discord channel type: text, text_thread, forum_post, voice, forum, stage, category
  channel_type text NOT NULL,
  parent_channel_id bigint REFERENCES channels(channel_id)
);

CREATE TABLE skills (
  id uuid PRIMARY KEY,

  name text NOT NULL UNIQUE,

  -- for skill similarity
  embedding vector
);

CREATE TABLE social_platforms (
  id uuid PRIMARY KEY,
  platform_name text NOT NULL UNIQUE,
  -- URL: https://strava.com https://spotify.com https://linkedin.com ...
  homepage text NOT NULL,
  -- public: can fetch with username only, oauth_required: needs user auth, unavailable: no API access
  access_type text NOT NULL
);



-- Depends on: channels, scores
CREATE TABLE messages (
  message_id bigint PRIMARY KEY,
  channel_id bigint NOT NULL REFERENCES channels(channel_id),

  sent_by bigint,

  content text NOT NULL,

  sent_at timestamptz NOT NULL,
  added_at timestamptz NOT NULL DEFAULT NOW(),
  last_edited timestamptz,
  deleted_at timestamptz,

  -- Self-reference for reply chains
  in_reply_to bigint REFERENCES messages(message_id),

  -- LLM personality score for only this message
  score_id uuid REFERENCES scores(id),

  -- Processing pipeline metadata
  triage_status text DEFAULT 'pending',    -- pending: Will be processed/processing: A worker is curretly processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
 -- |
 -- |
 -- ⌄
  is_significant boolean, -- Whether an LLM should score and extract personality from it, this field is also being set by an LLM TODO
 -- |
 -- |
 -- ⌄
  skill_status text, -- NULL: insignificant for skills/pending: Will be processed/processing: A worker is curretly processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
  personality_status text, -- NULL: insignificant for persinality extraction/pending: Will be processed/processing: A worker is curretly processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
  -- TODO make it so admins can manually override messages to be included/excluded from skills or personality processing
  processed_at timestamptz
);

-- ============================================================================
-- VESTIBULE USERS TABLE (canonical user entity)
-- A user can have 0 or many Discord accounts
-- ============================================================================

CREATE TABLE vestibule_users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  real_first text,
  real_last text,
  nickname text,

  intro_message_id bigint,  -- FK added via ALTER TABLE after messages exists

  -- Aggregated personality scores from all messages
  score_id uuid REFERENCES scores(id),
  score_last_updated timestamptz,

  -- Generated HEXACO diagram images
  current_diagram bytea,
  current_diagram_last_updated timestamptz,

  intro_diagram bytea
);

-- ============================================================================
-- DISCORD ACCOUNTS TABLE
-- Many-to-one: multiple discord accounts can belong to one vestibule_user
-- ============================================================================

CREATE TABLE discord_accounts (
  discord_user_id bigint PRIMARY KEY,
  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  username text NOT NULL UNIQUE,
  display_name text NOT NULL
);

ALTER TABLE messages 
  ADD CONSTRAINT fk_messages_sent_by 
  FOREIGN KEY (sent_by) REFERENCES discord_accounts(discord_user_id);

-- Add FK from vestibule_users to messages (resolves circular dependency)
ALTER TABLE vestibule_users
  ADD CONSTRAINT fk_vestibule_users_intro_message
  FOREIGN KEY (intro_message_id) REFERENCES messages(message_id);

-- ============================================================================
-- USER SKILLS TABLE
-- Depends on: vestibule_users, skills
-- ============================================================================

CREATE TABLE user_skills (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  skill_id uuid NOT NULL REFERENCES skills(id),

  -- Skill level 0-10
  level smallint NOT NULL CHECK (level >= 0 AND level <= 10),

  -- LLM reasoning context
  llm_context text,

  -- Prevent duplicate user-skill combinations
  UNIQUE (user_id, skill_id)
);

-- ============================================================================
-- USER SKILL EVIDENCE TABLE
-- Depends on: user_skills, messages
-- ============================================================================

CREATE TABLE user_skill_evidence (
  user_skill_id uuid NOT NULL REFERENCES user_skills(id),
  message_id bigint NOT NULL REFERENCES messages(message_id),

  -- How strongly this message supports the skill assessment
  weight real NOT NULL,
  reasoning text NOT NULL,

  PRIMARY KEY (user_skill_id, message_id)
);

-- ============================================================================
-- USER PLATFORM ASSOCIATION TABLE
-- Depends on: vestibule_users, messages, social_platforms
-- ============================================================================

CREATE TABLE user_platform_association (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  platform_id uuid NOT NULL REFERENCES social_platforms(id),

  platform_username text NOT NULL,
  platform_display_name text NOT NULL,
  profile_url text,

  -- Message where this association was mentioned/discovered
  platform_association_mention_message_id bigint REFERENCES messages(message_id),
  -- Reasoning for linking this user to this platform, TODO can be overriden to manually add associations by admins
  reasoning text NOT NULL,

  -- Sync scheduling
  last_synced_at timestamptz,
  last_sync_error text,
  sync_status text DEFAULT 'idle',         -- idle/running/failed

  -- Prevent duplicate user-platform combinations
  UNIQUE (vestibule_user_id, platform_id),
  CHECK (platform_association_mention_message_id IS NOT NULL OR reasoning IS NOT NULL)
);

-- ============================================================================
-- EXTERNAL CONTENT TABLE
-- Raw data fetched from external platforms (upserted on external_id)
-- Depends on: user_platform_association
-- ============================================================================
CREATE TABLE external_content (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  -- This is also the reason for fetching
  platform_association_id uuid NOT NULL REFERENCES user_platform_association(id),

  content_type text NOT NULL,              -- 'github_repo', 'strava_activity', 'linkedin_post', 'spotify_track'
  external_id text NOT NULL,               -- Platform's unique ID for this content

  raw_data jsonb NOT NULL,                 -- Full API response
  content_hash text,                       -- md5(raw_data) for change detection

  fetched_at timestamptz DEFAULT NOW(),
  updated_at timestamptz DEFAULT NOW(),

  UNIQUE (platform_association_id, content_type, external_id)
);

-- ============================================================================
-- USER ACTIVITIES TABLE
-- LLM-extracted activities from messages and external content (immutable)
-- Depends on: vestibule_users, external_content, messages
-- TODO make it so admins can manually add activites
-- ============================================================================
CREATE TABLE user_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,

  -- What kind of activity: 'run', 'cycle', 'swim', 'git_commit', 'took_job', 'add_experience', 'linkedin_post', 'song_listen', 'podcast_listen', 'skill', 'project', '' etc.
  activity_type text NOT NULL,
  -- Human-readable summary: "10km marathon", "rust-analyzer contribution", "Took job as Senior Engineer at Google"
  label text NOT NULL,

  -- When the activity happened (not when extracted)
  occurred_at timestamptz,

  -- Sources (at least one required)
  external_content_id uuid REFERENCES external_content(id),
  message_id bigint REFERENCES messages(message_id),

  reasoning text,

  CHECK (external_content_id IS NOT NULL OR message_id IS NOT NULL)
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Discord accounts
CREATE INDEX idx_discord_accounts_user ON discord_accounts(vestibule_user_id);

-- Messages: common query patterns
CREATE INDEX idx_messages_sent_by ON messages(sent_by);
CREATE INDEX idx_messages_channel_id ON messages(channel_id);
CREATE INDEX idx_messages_sent_at ON messages(sent_at DESC);
CREATE INDEX idx_messages_user_sent ON messages(sent_by, sent_at DESC);
CREATE INDEX idx_messages_channel_sent ON messages(channel_id, sent_at DESC);
CREATE INDEX idx_messages_in_reply_to ON messages(in_reply_to) WHERE in_reply_to IS NOT NULL;

-- Partial index for non-deleted messages
CREATE INDEX idx_messages_active ON messages(channel_id, sent_at DESC)
  WHERE deleted_at IS NULL;

-- Message processing pipeline
CREATE INDEX idx_messages_triage_pending ON messages(added_at)
  WHERE triage_status = 'pending';
CREATE INDEX idx_messages_skill_pending ON messages(added_at)
  WHERE skill_status = 'pending';
CREATE INDEX idx_messages_personality_pending ON messages(added_at)
  WHERE personality_status = 'pending';

-- User skills
CREATE INDEX idx_user_skills_user_id ON user_skills(user_id);

-- Platform associations and syncing
CREATE INDEX idx_user_platform_association_user_id ON user_platform_association(vestibule_user_id);
CREATE INDEX idx_user_platform_sync_due ON user_platform_association(next_sync_at)
  WHERE sync_status = 'idle';

-- User activities
CREATE INDEX idx_user_activities_user ON user_activities(user_id);
CREATE INDEX idx_user_activities_type ON user_activities(user_id, activity_type);
CREATE INDEX idx_user_activities_occurred ON user_activities(user_id, occurred_at DESC);

-- Vector similarity search (HNSW index)
-- CREATE INDEX idx_scores_embedding ON scores
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;

-- CREATE INDEX idx_skills_embedding ON skills
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;
