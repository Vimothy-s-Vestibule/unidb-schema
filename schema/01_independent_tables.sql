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
