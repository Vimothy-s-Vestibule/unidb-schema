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

CREATE TABLE discord_channels (
  channel_id bigint PRIMARY KEY,
  name text NOT NULL,
  -- discord channel type: text, text_thread, forum_post, voice, forum, stage, category
  channel_type text NOT NULL,
  parent_channel_id bigint REFERENCES discord_channels(channel_id)
);

CREATE TABLE skills (
  id uuid PRIMARY KEY,

  name text NOT NULL UNIQUE,

  -- for skill similarity
  embedding vector
);

-- For Things like profile pictures of discord accounts and Youtube commenter accounts/Youtube channel (inspired by seeing Reans' matching yt and discord pfps)
CREATE TABLE media_assets (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  -- HTML content-type
  content_type text NOT NULL,

  -- The actual content
  bytes text NOT NULL,

  -- For similarity with other media (TODO can we do across media types or only images <> images, etc.)
  embedding vector
);
