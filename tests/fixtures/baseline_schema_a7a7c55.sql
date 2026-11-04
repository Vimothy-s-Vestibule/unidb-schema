-- Baseline: schema/*.sql as of a7a7c55 (the unidb rev pinned by sentience-workers;
-- identical schema at main c20a146). Used by tests/migrations.rs as the 'old' schema
-- that existing databases were initialised with. Do not edit.
-- Load pgvector extension
CREATE EXTENSION IF NOT EXISTS "vector";

-- Default to public schema (should default to it anyway, but just to make sure)
SET search_path TO public;

-- Enums
CREATE TYPE discord_channel_type AS ENUM (
  'text', 'text_thread', 'public_thread', 'forum_post', 'voice', 'forum', 'stage', 'category'
);

CREATE TYPE activity_source AS ENUM (
  'llm_extraction', 'external_content', 'manual'
);

CREATE TYPE presence_status AS ENUM (
  'online', 'forced_online', 'absent', 'do_not_disturb', 'offline'
);

CREATE TYPE platform_access AS ENUM (
  'public', 'oauth_required', 'unavailable'
);

CREATE TYPE job_status AS ENUM (
  'pending', 'in_progress', 'completed', 'failed'
);

CREATE TYPE job_type AS ENUM (
  'youtube_channel_retrieval', 'discord_channel_sync', 'strava_retrieval', 'linkedin_scrape'
);

CREATE TYPE activity_record_type AS ENUM (
  'activity', 'fact', 'skill', 'emotion'
);

CREATE TYPE youtube_video_broadcast_status AS ENUM (
  'video', 'current_live', 'past_live', 'scheduled_live', 'none'
);
-- Tables without foreign keys go here

CREATE TABLE scores (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  -- HEXACO
  honesty double precision NOT NULL,
  emotionality double precision NOT NULL,
  extraversion double precision NOT NULL,
  agreeableness double precision NOT NULL,
  conscientiousness double precision NOT NULL,
  openness_to_experience double precision NOT NULL,

 -- Other interesting traits (experimental, TODO refine traits, consult with @no)
  agency double precision NOT NULL,
  achievement double precision NOT NULL,
  influence double precision NOT NULL,
  sarcasm double precision NOT NULL,
  security double precision NOT NULL,
  self_reflection double precision NOT NULL,
  technical_competence double precision NOT NULL,
  busyness double precision NOT NULL,

  -- Nice human-readable summary of what the user likes to talk about and what their personality is like (TODO independent of the score numbers!!!!)
  llm_summary text,

  embedding vector
);

CREATE TABLE discord_channels (
  channel_id bigint PRIMARY KEY,
  name text NOT NULL,
  channel_type discord_channel_type NOT NULL,
  parent_channel_id bigint REFERENCES discord_channels(channel_id)
);

-- For Things like profile pictures of discord accounts and Youtube commenter accounts/Youtube channel (inspired by seeing Reans' matching yt and discord pfps)
CREATE TABLE media_assets (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  -- HTML content-type
  content_type text NOT NULL,

  -- S3 object key
  -- Format: `<type>/<id>.<ext>` or similar.
  object_key text NOT NULL UNIQUE,

  -- Size of the asset in bytes (useful for caching logic/limits)
  size_bytes bigint,

  -- SHA-256 hash of the file content.
  content_hash text UNIQUE,

  -- Embedding Of the actual image for similarity with other media (TODO can we do across media types or only images <> images, etc.)
  embedding vector
);
-- Depends on: channels, scores
CREATE TABLE messages (
  message_id bigint PRIMARY KEY,
  channel_id bigint NOT NULL REFERENCES discord_channels(channel_id),

  sent_by bigint NOT NULL,

  content text NOT NULL,

  sent_at timestamptz NOT NULL,
  added_at timestamptz NOT NULL DEFAULT NOW(),
  last_edited timestamptz,
  deleted_at timestamptz,

  in_reply_to bigint REFERENCES messages(message_id)
);

CREATE TABLE message_edits (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id bigint NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
  old_content text NOT NULL,
  edited_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE message_attachments (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id bigint NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
  asset_id uuid NOT NULL REFERENCES media_assets(id) ON DELETE RESTRICT,
  
  added_at timestamptz NOT NULL DEFAULT NOW(),
  
  -- If the user edits the message and/or removes the attachment, we set deleted_at so we don't lose it for historical record
  deleted_at timestamptz
);
-- Canonical user entity
-- A user can have 0 to many Discord accounts
CREATE TABLE vestibule_users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  nickname text,

  -- FK added via ALTER TABLE after messages exists, It can be null if we add users from outside of the discord and because writing an intro is not mandatory anymore
  intro_message_id bigint,  

  -- Aggregated personality scores from all messages
  score_id uuid REFERENCES scores(id),
  score_last_updated timestamptz,

  -- Generated HEXACO diagram images
  current_diagram bytea,
  current_diagram_last_updated timestamptz,

  intro_diagram bytea
);


-- Multiple discord accounts can belong to one vestibule_user
-- TODO Save Discord connections such as LinkedIn, Spotify, Twitch, and others for big metadata gathering
CREATE TABLE discord_accounts (
  discord_user_id bigint PRIMARY KEY,
  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  username text NOT NULL UNIQUE,
  display_name text NOT NULL
);

INSERT INTO vestibule_users VALUES ('3d299903-2abf-4930-b2b9-8b083c1d21b2', 'Deleted User', NULL, NULL, NULL, NULL, NULL, NULL);
INSERT INTO discord_accounts VALUES ('0000000000', '3d299903-2abf-4930-b2b9-8b083c1d21b2', 'deleteduser', 'Deleted User');

-- FK from messages to discord_accounts (deferred due to dependency order)
ALTER TABLE messages 
  ADD CONSTRAINT fk_messages_sent_by 
  FOREIGN KEY (sent_by) REFERENCES discord_accounts(discord_user_id);

-- FK from vestibule_users to messages (deferred due to dependency order)
ALTER TABLE vestibule_users
  ADD CONSTRAINT fk_vestibule_users_intro_message
  FOREIGN KEY (intro_message_id) REFERENCES messages(message_id);
-- Depends on: messages, discord_accounts
CREATE TABLE discord_emojis (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  -- Unicode emoji string or custom emoji snowflake ID
  discord_emoji_id text NOT NULL,

  -- Some string if the emoji is a custom one created by some server, otherwise NULL
  from_guild text,

  -- Display name (useful for custom emojis)
  emoji_display_name text,

  -- Whether custom emoji is animated (GIF vs PNG) - only relevant when from_guild is not NULL
  is_animated boolean NOT NULL DEFAULT false,

  -- CDN URL for custom emojis (NULL for non-custom emojis)
  -- Format: TODO
  emoji_url text,

  asset_id uuid REFERENCES media_assets(id)
);

CREATE TABLE message_reactions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id bigint NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id),

  emoji_id uuid REFERENCES discord_emojis(id) ON DELETE RESTRICT,

  reacted_at timestamptz NOT NULL DEFAULT NOW(),

  -- Prevent duplicate reactions
  UNIQUE (message_id, user_id, emoji_id)
);
-- Depends on: discord_accounts, messages
CREATE TABLE discord_user_presence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  status presence_status,

  -- NULL when no activity is currently happening
  activity_type text,

  -- Activity details (only for presence_type = 'activity', matching serenity's Activity struct)
  name text,
  details text,
  state text,
  url text,

  started_at timestamptz NOT NULL DEFAULT NOW(),
  ended_at timestamptz,       -- NULL = current/still active

  -- If status = 'forced_online', we record the message that proved they were online
  evidence_message_id bigint REFERENCES messages(message_id) ON DELETE CASCADE,

  -- evidence_message_id is only allowed when status is 'forced_online'
  CHECK (
    evidence_message_id IS NULL OR status = 'forced_online'
  ),

  CONSTRAINT valid_presence_range CHECK (ended_at IS NULL OR ended_at > started_at)
);
CREATE TABLE social_platforms (
  id uuid PRIMARY KEY,
  platform_name text NOT NULL UNIQUE,
  -- URL: https://strava.com https://spotify.com https://linkedin.com ...
  homepage text NOT NULL,
  access_type platform_access NOT NULL,
  logo uuid REFERENCES media_assets(id)
);

-- Seed initial social platforms, more can be added anytime
INSERT INTO social_platforms (id, platform_name, homepage, access_type)
VALUES
  (gen_random_uuid(), 'Strava', 'https://strava.com', 'oauth_required'),
  (gen_random_uuid(), 'Spotify', 'https://spotify.com', 'oauth_required'),
  (gen_random_uuid(), 'LinkedIn', 'https://linkedin.com', 'unavailable'),
  (gen_random_uuid(), 'YouTube', 'https://youtube.com', 'public'),
  -- TODO curl -L -X GET "https://api.github.com/users/alex" -H "Accept: application/vnd.github+json"
  (gen_random_uuid(), 'GitHub', 'https://github.com', 'public'),
  -- TODO Use Nvidia parakeet to transscribe Sylvan's daily internal monologue and run analysis on it for maximum data extraction: https://vimothee.substack.com/feed
  (gen_random_uuid(), 'Substack', 'https://substack.com', 'public')
ON CONFLICT (platform_name) DO NOTHING;

-- TODO seed db with vestibule channel, reans, big smoke, tokitsuno and all others who have their youtube connected to their discord or "watch my latest video:" in their discord bio
-- Depends on: vestibule_users, platforms, messages
CREATE TABLE connected_accounts (
  -- Vestibule-specific internal UUID
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  -- With which user the account is associated
  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  -- UUID of the platform, see social_platforms
  platform_id uuid NOT NULL REFERENCES social_platforms(id),

  -- For YT: The Channel ID
  -- Might be NULL for platforms like GitHub
  platform_user_id text,

  -- For YT: Sylvan Franklin
  platform_display_name text NOT NULL,

  -- For YT: @SylvanFranklin
  platform_username text NOT NULL,

  bio text,
  -- On Youtube, for example, some people fill in where they are located in the world or some toher stuff
  bio_additional_info text,
  
  profile_picture uuid REFERENCES media_assets(id),


  -- For finding similar users in Discord accounts and across other platforms, TODO choose universal vector dimensions amount and choose embedding model (preferably selfhostable)
  -- TODO Embed both username and displayname to create 2 "maps" of these to see what matches
  name_embedding vector,


  -- Metadata about how we found out
  mention_message_id bigint REFERENCES messages(message_id),
  -- Or if linked manually, the reason why we believe the account is linked to the discord user
  reasoning text NOT NULL,



  UNIQUE (vestibule_user_id, platform_id, platform_username),
  CHECK (mention_message_id IS NOT NULL OR reasoning IS NOT NULL)
);
CREATE TABLE youtube_videos (
  -- The part after youtube.com/watch?v=
  video_id text PRIMARY KEY,

  channel_vestibule_id uuid NOT NULL REFERENCES connected_accounts(id) ON DELETE CASCADE,
  title text NOT NULL,
  description text,
  published_at timestamptz NOT NULL,
  total_views bigint NOT NULL DEFAULT 0,
  total_likes bigint NOT NULL DEFAULT 0,
  total_comments bigint NOT NULL DEFAULT 0,
  keyword_tags text[],

  -- Content Details
  duration_seconds integer NOT NULL,
  broadcast_status youtube_video_broadcast_status NOT NULL, -- Values: 'video', 'current_live', 'past_live', 'scheduled_live', 'none'
  
  -- Media & Discovery
  thumbnail_asset_id uuid REFERENCES media_assets(id),
  transcript_asset_id uuid REFERENCES media_assets(id),
  -- Downloaded audio ID
  audio_asset_id uuid REFERENCES media_assets(id),

  added_at timestamptz DEFAULT NOW()
);

CREATE TABLE youtube_comments (
  comment_id text PRIMARY KEY,
  video_id text NOT NULL REFERENCES youtube_videos(video_id) ON DELETE CASCADE,
  
  -- The channel that posted the comment
  author_raw_channel_id text NOT NULL,
  -- If the channel is already in the db, populate this field too, TODO When and how do we decide If we want to add a user to the db, I'd say after the second comment they wrute or if they are deemed significant and can be linked to a vestibule user by an admin.
  author_channel_id uuid REFERENCES connected_accounts(id) ON DELETE CASCADE,

  content text NOT NULL,
  
  like_count integer NOT NULL,
  
  published_at timestamptz NOT NULL,
  edited_at timestamptz,
  
  -- Self-reference for replies
  in_reply_to text REFERENCES youtube_comments(comment_id),

  added_at timestamptz
);
CREATE TABLE jobs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  
  -- Job metadata
  job_type job_type NOT NULL, -- 'youtube_channel_scrape', 'discord_channel_scrape', 'strava_sync', etc.
  status job_status NOT NULL DEFAULT 'pending', -- 'pending', 'in_progress', 'completed', 'failed'

  -- Worker tracking, set both to NULL after
  locked_by_worker_id text, -- ID of the worker currently processing this
  locked_at timestamptz,

  -- Scheduling and retries
  scheduled_for timestamptz NOT NULL DEFAULT NOW(),
  attempts int NOT NULL DEFAULT 0,
  max_attempts int NOT NULL DEFAULT 3,
  last_error text,

  -- POLYMORPHIC FOREIGN KEYS 
  -- (Only ONE of these should be NOT NULL for a given row)
  youtube_video_id text REFERENCES youtube_videos(video_id) ON DELETE CASCADE,
  discord_channel_id bigint REFERENCES discord_channels(channel_id) ON DELETE CASCADE,
  connected_account_id uuid REFERENCES connected_accounts(id) ON DELETE CASCADE,
  media_asset_id uuid REFERENCES media_assets(id) ON DELETE CASCADE,

  -- Check constraint to ensure exactly one target table is specified
  CONSTRAINT has_single_target CHECK (
    (youtube_video_id IS NOT NULL)::int + 
    (discord_channel_id IS NOT NULL)::int + 
    (connected_account_id IS NOT NULL)::int +
    (media_asset_id IS NOT NULL)::int = 1
  )
);
-- Depends on: vestibule_users, external_content, messages, youtube_comments
CREATE TABLE user_facts_and_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,

  -- Is this an 'activity' (has timeline/duration), a 'fact' (stateful knowledge), or a 'skill'
  record_type activity_record_type NOT NULL,

  -- Where this came from: 'llm_extraction', 'external_content', 'manual'
  source activity_source NOT NULL,

  -- If this includes some user generated content, this is the LLM eval of that
  score_id uuid REFERENCES scores(id),

  -- Type: 'took_job', 'timezone', 'location', 'primary_machine', 'editor', 'real_first' (real first name), 'real_last' (real last name), 'favorite_car', ..., OR for skills: the skill name like 'Rust', 'Guitar'
  type text NOT NULL,
  
  -- Value: "10km marathon", "UTC+2", "Neovim", "Senior Software Engineer @ Google".
  value text NOT NULL,

  -- Confidence that the fact is actually true or activity is accurate and was interpreted correctly by the LLM
  confidence real NOT NULL DEFAULT 1.0,

  -- For 'skill' only: Proficiency level (0-10)
  level smallint CHECK (level >= 0 AND level <= 10),

  -- For 'activity' only: When it happened
  started_at timestamptz,
  ended_at timestamptz,

  -- For 'fact' or 'skill' only: Is this still true?
  is_current boolean DEFAULT true,

  created_at timestamptz NOT NULL DEFAULT NOW(),

  -- The embedding of a combination of the type and value of the fact/skill, useful to see e.g.:  Person 1 took a Junior swe position at google while person 2 interned there aswell
  type_value_embedding vector

);

CREATE TABLE fact_extraction_attempts (
      message_id bigint PRIMARY KEY REFERENCES messages(message_id),
      attempted_at timestamptz NOT NULL DEFAULT NOW()
);
-- Depends on: user_facts_and_activities, messages, youtube_comments, external_content, discord_user_presence

-- Raw data fetched from external platforms
-- Depends on: connected_accounts
CREATE TABLE external_content (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id uuid NOT NULL REFERENCES connected_accounts(id) ON DELETE CASCADE,

  content_type text NOT NULL,  -- 'github_repo', 'strava_activity', 'spotify_track' (NOT youtube — use youtube_videos/youtube_comments tables instead)


  raw_data jsonb NOT NULL,  -- Full API response
  content_hash text,  -- md5(raw_data) for change detection (TODO maybe choose faster, better algo)

  fetched_at timestamptz NOT NULL DEFAULT NOW(),
  -- If it was fetched multiple times, when it was fetched the last time
  updated_at timestamptz,

  UNIQUE (account_id, content_hash),
  -- YouTube has dedicated tables: youtube_videos, youtube_comments
  CHECK (content_type NOT IN ('youtube_video', 'youtube_comment', 'youtube_channel'))
);

CREATE TABLE fact_and_activity_evidence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  
  fact_or_activity_id uuid NOT NULL REFERENCES user_facts_and_activities(id) ON DELETE CASCADE,
  
  -- The source of the evidence (Polymorphic FKs)
  message_id bigint REFERENCES messages(message_id) ON DELETE CASCADE,
  youtube_comment_id text REFERENCES youtube_comments(comment_id) ON DELETE CASCADE,
  external_content_id uuid REFERENCES external_content(id) ON DELETE CASCADE,
  discord_presence_id uuid REFERENCES discord_user_presence(id) ON DELETE CASCADE,

  -- How strongly this evidence supports the fact or activity (0.0 - 1.0)
  weight real NOT NULL DEFAULT 1.0,
  
  -- The LLM's explanation of why this specific source supports the fact/activity
  reasoning text NOT NULL,

  -- Ensure exactly one source is provided per evidence row
  CONSTRAINT has_single_evidence_source CHECK (
    (message_id IS NOT NULL)::int + 
    (youtube_comment_id IS NOT NULL)::int + 
    (external_content_id IS NOT NULL)::int +
    (discord_presence_id IS NOT NULL)::int = 1
  ),

  -- Prevent duplicate evidence for the exact same fact/source combo
  UNIQUE NULLS NOT DISTINCT (fact_or_activity_id, message_id, youtube_comment_id, external_content_id, discord_presence_id)
);


CREATE OR REPLACE FUNCTION verify_fact_has_evidence()
RETURNS trigger AS $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM fact_and_activity_evidence WHERE fact_or_activity_id = NEW.id) THEN
    RAISE EXCEPTION 'A fact or activity must have at least one supporting evidence record already inserted. (Fact/Activity ID: %)', NEW.id;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- INSERT the fact, then INSERT the evidence, and only then does Postgres check.
CREATE CONSTRAINT TRIGGER ensure_fact_has_evidence
AFTER INSERT ON user_facts_and_activities
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION verify_fact_has_evidence();
CREATE TABLE topic (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- LLM-extracted concise summary of what the topic messages discuss
    name text NOT NULL,
    embedding vector
);


CREATE TABLE topic_message_relation (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id uuid REFERENCES topic(id) NOT NULL,
    message_id bigint REFERENCES messages(message_id) NOT NULL
);

-- Tracks which messages the topic-sorter has attempted to classify.
-- Presence of a row = processed; no row = not yet tried.
-- A message with no topic_message_relation row was processed but assigned no topic.
CREATE TABLE message_classification_attempts (
    message_id bigint PRIMARY KEY REFERENCES messages(message_id),
    attempted_at timestamptz NOT NULL DEFAULT NOW()
);
-- Discord accounts
CREATE INDEX idx_discord_accounts_user ON discord_accounts(vestibule_user_id);

-- Message reactions
CREATE INDEX idx_message_reactions_message ON message_reactions(message_id);
CREATE INDEX idx_message_reactions_user ON message_reactions(user_id);

-- User presence: current status per user
CREATE INDEX idx_presence_current_status ON discord_user_presence(user_id)
  WHERE ended_at IS NULL AND status IS NOT NULL;
-- User presence: current activities per user
CREATE INDEX idx_presence_current_activity ON discord_user_presence(user_id)
  WHERE ended_at IS NULL AND activity_type IS NOT NULL;
-- User presence: enabling historical queries "who was online at time X"
CREATE INDEX idx_presence_history ON discord_user_presence(user_id, started_at, ended_at);
-- User presence: time-based queries
CREATE INDEX idx_presence_time ON discord_user_presence(started_at DESC);

-- Messages: common queries
CREATE INDEX idx_messages_sent_by ON messages(sent_by);
CREATE INDEX idx_messages_channel_id ON messages(channel_id);
CREATE INDEX idx_messages_sent_at ON messages(sent_at DESC);
CREATE INDEX idx_messages_user_sent ON messages(sent_by, sent_at DESC);
CREATE INDEX idx_messages_channel_sent ON messages(channel_id, sent_at DESC);
CREATE INDEX idx_messages_in_reply_to ON messages(in_reply_to) WHERE in_reply_to IS NOT NULL;

-- Partial index for non-deleted messages
CREATE INDEX idx_messages_active ON messages(channel_id, sent_at DESC)
  WHERE deleted_at IS NULL;

-- Message edits
CREATE INDEX idx_message_edits_message_id ON message_edits(message_id, edited_at ASC);

-- Partial index for active message attachments
CREATE INDEX idx_message_attachments_active ON message_attachments(message_id)
  WHERE deleted_at IS NULL;
CREATE INDEX idx_message_attachments_asset_id ON message_attachments(asset_id);


-- Connected accounts
CREATE INDEX idx_connected_accounts_user_id ON connected_accounts(vestibule_user_id);

-- User activities
CREATE INDEX idx_user_facts_and_activities_user ON user_facts_and_activities(user_id);
CREATE INDEX idx_user_facts_and_activities_type ON user_facts_and_activities(user_id, type);
CREATE INDEX idx_user_facts_and_activities_started ON user_facts_and_activities(user_id, started_at DESC) WHERE record_type = 'activity';
CREATE INDEX idx_user_facts_and_activities_current ON user_facts_and_activities(user_id, is_current) WHERE record_type = 'fact';

-- Vector similarity search (HNSW index)
-- CREATE INDEX idx_scores_embedding ON scores
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;

-- CREATE INDEX idx_skills_embedding ON skills
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;

-- YouTube tables
CREATE INDEX idx_youtube_comments_video ON youtube_comments(video_id);
CREATE INDEX idx_youtube_comments_author ON youtube_comments(author_channel_id);
CREATE INDEX idx_youtube_comments_published ON youtube_comments(published_at DESC);
