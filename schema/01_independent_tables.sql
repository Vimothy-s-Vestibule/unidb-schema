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
  -- public: can fetch with username/public data only, oauth_required: needs user auth, unavailable: no API access, but still added because mentioned by user(s)
  access_type text NOT NULL
);
