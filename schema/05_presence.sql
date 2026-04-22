-- Depends on: discord_accounts
-- Unified presence table: tracks both online/offline status changes AND rich presence activities
-- Event-sourced: insert when status/activity starts, update ended_at when it ends
CREATE TABLE discord_user_presence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  -- What kind of presence record this is
  -- 'status': online/offline status change
  -- 'activity': rich presence activity
  presence_type text NOT NULL,

  -- For presence_type = 'status': online, forced_online, absent, do_not_disturb, offline
  -- NULL for activity rows
  status text,

  -- For presence_type = 'activity': Maps to serenity's ActivityType (Playing, Streaming, Listening, Watching, Custom, Competing)
  -- NULL for status change rows
  activity_type text,

  -- Activity details (only for presence_type = 'activity', matching serenity's Activity struct)
  name text,
  details text,
  state text,
  url text,

  started_at timestamptz NOT NULL DEFAULT NOW(),
  ended_at timestamptz,         -- NULL = current/still active

  -- Status rows must have status, activity rows must have activity_type + name
  CHECK (
    (presence_type = 'status' AND status IS NOT NULL AND activity_type IS NULL) OR
    (presence_type = 'activity' AND activity_type IS NOT NULL AND name IS NOT NULL AND status IS NULL)
  ),

  CONSTRAINT valid_presence_range CHECK (ended_at IS NULL OR ended_at > started_at)
);

CREATE TABLE forced_online_evidence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,
  message_id bigint NOT NULL UNIQUE REFERENCES messages(message_id) ON DELETE CASCADE,
  presence_id uuid NOT NULL REFERENCES discord_user_presence(id),

  detected_at timestamptz NOT NULL DEFAULT NOW()
);
