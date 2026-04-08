-- ============================================================================
-- USER PRESENCE TABLE
-- Tracks Discord online/offline status changes (event-sourced)
-- Depends on: discord_accounts
-- ============================================================================

CREATE TABLE user_presence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  -- Core status: online, idle, dnd, offline, invisible
  status text NOT NULL,

  -- Per-client breakdown (if available from Discord)
  desktop_status text,
  mobile_status text,
  web_status text,

  -- Time range this status was active
  started_at timestamptz NOT NULL DEFAULT NOW(),
  ended_at timestamptz,  -- NULL = current status

  CONSTRAINT valid_presence_range CHECK (ended_at IS NULL OR ended_at > started_at)
);

-- ============================================================================
-- USER PRESENCE ACTIVITIES TABLE
-- Tracks rich presence / activities (games, Spotify, streaming, etc.)
-- Event-sourced: only insert when activity starts, update ended_at when it ends
-- Depends on: discord_accounts
-- ============================================================================

CREATE TABLE user_presence_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  -- Activity type: playing, streaming, listening, watching, competing, custom
  activity_type text NOT NULL,

  -- Activity details
  name text NOT NULL,           -- Game name, song title, custom status text, etc.
  details text,                 -- Secondary line (e.g., "In Menu", "by Artist")
  state text,                   -- Third line (e.g., "Playing Solo", "In a party")

  -- For streaming/music
  url text,                     -- Stream URL, Spotify link, etc.

  -- Images if available
  large_image_url text,
  small_image_url text,

  -- Time range
  started_at timestamptz NOT NULL DEFAULT NOW(),
  ended_at timestamptz,         -- NULL = still active

  -- Store any extra data we might want later
  raw_data jsonb,

  CONSTRAINT valid_activity_range CHECK (ended_at IS NULL OR ended_at > started_at)
);

-- ============================================================================
-- FORCED ONLINE EVIDENCE TABLE
-- Links messages sent while user showed as offline/invisible
-- Depends on: discord_accounts, messages, user_presence
-- ============================================================================

CREATE TABLE forced_online_evidence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,
  message_id bigint NOT NULL UNIQUE REFERENCES messages(message_id) ON DELETE CASCADE,

  -- What status they were showing when they sent the message
  displayed_status text NOT NULL,  -- offline, invisible

  -- Link to the presence record at that time
  presence_id uuid REFERENCES user_presence(id) ON DELETE SET NULL,

  detected_at timestamptz NOT NULL DEFAULT NOW()
);
