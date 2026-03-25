-- ============================================================================
-- UniDB Schema
-- Run against a blank PostgreSQL database to initialize all tables
-- Tables are ordered to satisfy foreign key dependencies
-- TODO make the LLM relfect on the comments in this schema so it knows what it is even grading by.
-- ============================================================================

-- Load vector extension
CREATE EXTENSION IF NOT EXISTS "vector";

-- Default to public schema
SET search_path TO public;

-- ============================================================================
-- INDEPENDENT TABLES (no foreign keys)
-- ============================================================================

CREATE TABLE scores (
  score_id text PRIMARY KEY,

  -- ============================================================================
  -- HEXACO PERSONALITY TRAITS (0.0 - 1.0 scale)
  -- Based on the HEXACO model of personality structure, some additional fields added for experimentation
  -- ============================================================================

  -- HEXACO Honesty-Humility: Sincerity, fairness, lack of greed/entitlement.
  -- High: Genuine, doesn't manipulate or exploit, modest, not status-driven
  -- Low: Flatters for gain, bends rules for advantage, feels entitled, status-seeking
  honesty double precision NOT NULL,

  -- HEXACO Emotionality: Emotional reactivity, volatility, anxiety.
  -- High: Gets heated easily, expresses strong emotions, anxious, reactive to stress
  -- Low: Calm under pressure, emotionally stable, rarely gets worked up
  emotionality double precision NOT NULL,

  -- HEXACO Extraversion: Social confidence, enthusiasm, enjoyment of interaction.
  -- High: Initiates conversations, energetic, confident, enjoys group discussions
  -- Low: Quiet, reserved, avoids spotlight, prefers observing over participating
  extraversion double precision NOT NULL,

  -- HEXACO Agreeableness: Forgiveness, gentleness, patience, willingness to compromise.
  -- High: Lets slights go, avoids conflict, patient with others, flexible
  -- Low: Holds grudges, critical, quick to argue, stubborn in disagreements
  agreeableness double precision NOT NULL,

  -- HEXACO Conscientiousness: Organization, diligence, perfectionism, prudence.
  -- High: Plans carefully, thorough, disciplined, considers consequences
  -- Low: Disorganized, impulsive, cuts corners, acts without planning
  conscientiousness double precision NOT NULL,

  -- HEXACO Openness: Curiosity, creativity, aesthetic appreciation, unconventionality.
  -- High: Explores ideas, appreciates art/beauty, creative, embraces unusual concepts
  -- Low: Practical-focused, conventional, uninterested in abstract/artistic topics
  openness_to_experience double precision NOT NULL,


  -- Agency: Follow-through on stated commitments and self-initiated action.
  -- High: Does what they say, self-directed, drives progress and works on projects proactively
  -- Low: Makes promises but doesn't deliver, passive, waits for others
  agency double precision NOT NULL,

  -- Achievement: Track record of completed, impactful projects or accomplishments.
  -- High: Has shipped projects, can point to concrete outcomes
  -- Low: Many started projects, few finished; talks about ideas without execution
  achievement double precision NOT NULL,

  -- Influence: Social influence and authority in conversations (behavioral, not role-based).
  -- High: Others defer to their opinions, shapes discussion direction, respected
  -- Low: Opinions ignored, follows rather than leads discussions
  influence double precision NOT NULL,

  -- Sarcasm: Frequency and intensity of ironic/sarcastic communication.
  -- High: Often says the opposite of what they mean, dry humor, mocking tone
  -- Low: Direct, literal communication, rare irony
  sarcasm double precision NOT NULL,

  -- Security (self-confidence): Certainty vs. self-doubt in communication.
  -- High: Confident assertions, rarely hedges, owns their opinions
  -- Low: Frequently hedges ("I think maybe..."), seeks validation, self-deprecating
  security double precision NOT NULL,

  -- Self-reflection: Explicit reconsideration of own beliefs, decisions, or growth.
  -- High: "I used to think X but now...", acknowledges mistakes, updates views
  -- Low: Never revisits past positions, doesn't discuss personal growth
  -- Note: Distinguish from insecurity—self-reflection is about growth, not doubt
  self_reflection double precision NOT NULL,

  -- Technical competence: Quality of technical reasoning/solutions demonstrated.
  -- High: Correct, nuanced technical explanations; solves problems efficiently
  -- Low: Frequent errors, surface-level understanding, needs correction often
  technical_competence double precision NOT NULL,

  -- Busyness: How occupied the person appears with projects, work, or life obligations.
  -- High: Frequently mentions being busy, many concurrent commitments, limited availability
  -- Low: Appears to have free time, few mentioned obligations, readily available
  busyness double precision NOT NULL,


  -- Vector embedding for personality similarity search
  embedding vector
);

CREATE TABLE channels (
  channel_id bigint PRIMARY KEY,
  name text NOT NULL,
  -- discord channel type: text, voice, forum, text_thread, forum_post, stage, category
  channel_type text NOT NULL,
  parent_channel_id bigint REFERENCES channels(channel_id)
);

CREATE TABLE skills (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL UNIQUE,
  -- Vector embedding for skill similarity
  embedding vector
);

CREATE TABLE social_platforms (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  platform_name text NOT NULL UNIQUE,
  homepage text NOT NULL,
  -- public: can fetch with username only, oauth_required: needs user auth, unavailable: no API access
  access_type text NOT NULL DEFAULT 'public'
);

-- ============================================================================
-- MESSAGES TABLE
-- Depends on: channels, scores
-- ============================================================================

CREATE TABLE messages (
  message_id bigint PRIMARY KEY,
  channel_id bigint NOT NULL REFERENCES channels(channel_id),

  user_id bigint NOT NULL,
  content text NOT NULL,

  sent_at timestamptz NOT NULL,
  added_at timestamptz NOT NULL DEFAULT NOW(),
  last_edited timestamptz,
  deleted_at timestamptz,

  -- Self-reference for reply chains
  in_reply_to bigint REFERENCES messages(message_id),

  -- LLM personality score for only this message
  score_id text REFERENCES scores(score_id),

  -- Processing pipeline status
  triage_status text DEFAULT 'pending',    -- pending/processing/complete/skipped/failed
  is_significant boolean,
  skill_status text,                        -- NULL/pending/processing/complete/failed
  personality_status text,                  -- NULL/pending/processing/complete/failed
  processed_at timestamptz
);

-- ============================================================================
-- VESTIBULE USERS TABLE
-- Depends on: messages, scores
-- ============================================================================

CREATE TABLE vestibule_users (
  discord_user_id bigint PRIMARY KEY,
  discord_username text NOT NULL UNIQUE,
  discord_display_name text NOT NULL,

  -- pending/sending/sent
  status text NOT NULL,

  intro_message_id bigint REFERENCES messages(message_id),

  -- Aggregated personality score
  score_id text REFERENCES scores(score_id),
  score_last_updated timestamptz,

  -- Generated diagram images
  current_diagram bytea,
  current_diagram_last_updated timestamptz,
  intro_diagram bytea,

  -- Periodic aggregation scheduling
  aggregate_interval_hours int DEFAULT 24,
  next_aggregate_at timestamptz,
  last_aggregated_at timestamptz
);

-- ============================================================================
-- USER SKILLS TABLE
-- Depends on: vestibule_users, skills
-- ============================================================================

CREATE TABLE user_skills (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  user_id bigint NOT NULL REFERENCES vestibule_users(discord_user_id),
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

  vestibule_user_id bigint NOT NULL REFERENCES vestibule_users(discord_user_id),
  platform_id uuid NOT NULL REFERENCES social_platforms(id),

  platform_username text NOT NULL,
  platform_display_name text NOT NULL,
  profile_url text,

  -- Message where this association was mentioned/discovered
  platform_association_mention_message_id bigint NOT NULL REFERENCES messages(message_id),
  reasoning text NOT NULL,

  -- Sync scheduling
  sync_interval_hours int DEFAULT 24,
  next_sync_at timestamptz DEFAULT NOW(),
  last_synced_at timestamptz,
  sync_status text DEFAULT 'idle',         -- idle/running/failed
  last_sync_error text,

  -- Prevent duplicate user-platform combinations
  UNIQUE (vestibule_user_id, platform_id)
);

-- ============================================================================
-- EXTERNAL CONTENT TABLE
-- Raw data fetched from external platforms (upserted on external_id)
-- Depends on: user_platform_association
-- ============================================================================

CREATE TABLE external_content (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  platform_association_id uuid NOT NULL REFERENCES user_platform_association(id),

  content_type text NOT NULL,              -- 'github_repo', 'strava_activity', 'linkedin_post', 'spotify_track'
  external_id text NOT NULL,               -- Platform's unique ID for this content

  raw_data jsonb NOT NULL,                 -- Full API response
  content_hash text,                       -- md5(raw_data) for change detection

  fetched_at timestamptz DEFAULT NOW(),
  updated_at timestamptz DEFAULT NOW(),

  -- Processing pipeline status (same as messages)
  triage_status text DEFAULT 'pending',
  is_significant boolean,
  skill_status text,
  personality_status text,
  processed_at timestamptz,

  UNIQUE (platform_association_id, content_type, external_id)
);

-- ============================================================================
-- USER ACTIVITIES TABLE
-- LLM-extracted activities from messages and external content (immutable)
-- Depends on: vestibule_users, external_content, messages
-- ============================================================================

CREATE TABLE user_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES vestibule_users(discord_user_id),

  -- What kind of activity: 'run', 'commit', 'job', 'listen', 'skill', 'project', etc.
  activity_type text NOT NULL,
  -- Human-readable summary: "10km marathon", "rust-analyzer contribution", "Senior Engineer at Google"
  label text NOT NULL,
  -- Type-specific structured data (distance_km, repo_url, company, track_id, etc.)
  data jsonb,
  -- When the activity happened (not when extracted)
  occurred_at timestamptz,

  -- Sources (at least one required)
  external_content_id uuid REFERENCES external_content(id),
  message_id bigint REFERENCES messages(message_id),

  confidence real,
  reasoning text,
  extracted_at timestamptz DEFAULT NOW(),

  CHECK (external_content_id IS NOT NULL OR message_id IS NOT NULL)
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Messages: common query patterns
CREATE INDEX idx_messages_user_id ON messages(user_id);
CREATE INDEX idx_messages_channel_id ON messages(channel_id);
CREATE INDEX idx_messages_sent_at ON messages(sent_at DESC);
CREATE INDEX idx_messages_user_sent ON messages(user_id, sent_at DESC);
CREATE INDEX idx_messages_channel_sent ON messages(channel_id, sent_at DESC);

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

-- External content processing
CREATE INDEX idx_external_content_platform ON external_content(platform_association_id);
CREATE INDEX idx_external_content_triage_pending ON external_content(fetched_at)
  WHERE triage_status = 'pending';
CREATE INDEX idx_external_content_skill_pending ON external_content(fetched_at)
  WHERE skill_status = 'pending';
CREATE INDEX idx_external_content_personality_pending ON external_content(fetched_at)
  WHERE personality_status = 'pending';

-- User activities
CREATE INDEX idx_user_activities_user ON user_activities(user_id);
CREATE INDEX idx_user_activities_type ON user_activities(user_id, activity_type);
CREATE INDEX idx_user_activities_occurred ON user_activities(user_id, occurred_at DESC);

-- User aggregation scheduling
CREATE INDEX idx_users_aggregate_due ON vestibule_users(next_aggregate_at)
  WHERE next_aggregate_at IS NOT NULL;

-- Vector similarity search (HNSW index)
CREATE INDEX idx_scores_embedding ON scores
  USING hnsw (embedding vector_cosine_ops)
  WHERE embedding IS NOT NULL;

CREATE INDEX idx_skills_embedding ON skills
  USING hnsw (embedding vector_cosine_ops)
  WHERE embedding IS NOT NULL;
