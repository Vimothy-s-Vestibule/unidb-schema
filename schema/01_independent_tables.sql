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

  embedding vector,

  -- scoring_prompt() scores every trait on a 0.0..=1.0 scale (rejects NaN too)
  CONSTRAINT scores_values_in_unit_range CHECK (
    honesty BETWEEN 0 AND 1 AND emotionality BETWEEN 0 AND 1 AND
    extraversion BETWEEN 0 AND 1 AND agreeableness BETWEEN 0 AND 1 AND
    conscientiousness BETWEEN 0 AND 1 AND openness_to_experience BETWEEN 0 AND 1 AND
    agency BETWEEN 0 AND 1 AND achievement BETWEEN 0 AND 1 AND
    influence BETWEEN 0 AND 1 AND sarcasm BETWEEN 0 AND 1 AND
    security BETWEEN 0 AND 1 AND self_reflection BETWEEN 0 AND 1 AND
    technical_competence BETWEEN 0 AND 1 AND busyness BETWEEN 0 AND 1
  )
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
