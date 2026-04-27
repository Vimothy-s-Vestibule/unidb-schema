-- Load pgvector extension
CREATE EXTENSION IF NOT EXISTS "vector";

-- Default to public schema (should default to it anyway, but just to make sure)
SET search_path TO public;

-- Enums
CREATE TYPE discord_channel_type AS ENUM (
  'text', 'text_thread', 'forum_post', 'voice', 'forum', 'stage', 'category'
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
  'activity', 'fact', 'skill'
);

CREATE TYPE youtube_video_broadcast_status AS ENUM (
  'video', 'current_live', 'past_live', 'scheduled_live', 'none'
);
