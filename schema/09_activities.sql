-- Unified activity table: both LLM-extracted activities and Discord rich presence activities
-- Depends on: vestibule_users, discord_accounts, external_content, messages, youtube_comments
CREATE TABLE user_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  -- Where this activity came from
  -- 'discord_presence': real-time Discord rich presence (games, Spotify, streaming, etc.)
  -- 'llm_extraction': LLM-extracted from a message
  -- 'external_content': Obtained from the external_content table
  -- 'manual': manually added by an admin
  source text NOT NULL,

  -- For discord_presence activities: the discord account that was observed
  -- NULL for llm_extraction/manual activities (use user_id instead)
  discord_user_id bigint REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  -- For llm_extraction/manual activities: the vestibule user
  -- NULL for discord_presence activities (use discord_user_id instead)
  user_id uuid REFERENCES vestibule_users(id) ON DELETE CASCADE,

  -- What kind of activity: 'run', 'cycle', 'swim', 'git_commit', 'took_job', 'game', 'spotify', 'streaming', 'custom_status', etc.
  activity_type text NOT NULL,
  -- Human-readable summary: "10km marathon", "rust-analyzer contribution", "Playing Elden Ring"
  label text NOT NULL,

  -- Rich presence detail fields (from Discord presence, NULL for other things)
  details text,                 -- Secondary line (e.g., "In Menu", "by Artist")
  state text,                   -- Third line (e.g., "Playing Solo", "In a party")
  url text,                     -- Stream URL, Spotify link, etc.

  -- When the activity happened/started (not when extracted)
  started_at timestamptz,
  -- When the activity ended (NULL for point-in-time activities or still ongoing)
  ended_at timestamptz,

  -- Sources for LLM-extracted activities (at least one required when source = 'llm_extraction')
  external_content_id uuid REFERENCES external_content(id),
  message_id bigint REFERENCES messages(message_id),
  youtube_comment_id text REFERENCES youtube_comments(comment_id),
  discord_presence_id uuid REFERENCES user_presence(id),
  -- And/or
  reasoning text,

  -- Raw Discord presence data (only for source = 'discord_presence')
  raw_data jsonb,

  -- Exactly one of discord_user_id or user_id must be set
  CHECK (
    (discord_user_id IS NOT NULL AND user_id IS NULL) OR
    (discord_user_id IS NULL AND user_id IS NOT NULL)
  ),

  -- LLM-extracted activities must have at least one source
  CHECK (
    source != 'llm_extraction' OR
    (external_content_id IS NOT NULL OR message_id IS NOT NULL OR youtube_comment_id IS NOT NULL)
  ),

  -- Duration constraint
  CONSTRAINT valid_activity_range CHECK (ended_at IS NULL OR ended_at > started_at)
);
