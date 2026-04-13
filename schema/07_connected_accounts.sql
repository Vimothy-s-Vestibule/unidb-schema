-- ============================================================================
-- CONNECTED ACCOUNTS TABLE
-- Tracks a user's linked third-party accounts (GitHub, Spotify, etc.)
-- Depends on: vestibule_users, platforms, messages
-- ============================================================================

CREATE TABLE connected_accounts (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  platform_id uuid NOT NULL REFERENCES social_platforms(id),

  -- Platform info
  platform_username text NOT NULL,
  platform_display_name text NOT NULL,
  profile_url text,

  -- Metadata about how we found out
  mention_message_id bigint REFERENCES messages(message_id),
  reasoning text NOT NULL,

  -- Sync scheduling
  last_synced_at timestamptz,
  last_sync_error text,
  sync_status text NOT NULL DEFAULT 'idle',

  UNIQUE (vestibule_user_id, platform_id),
  CHECK (mention_message_id IS NOT NULL OR reasoning IS NOT NULL)
);

-- ============================================================================
-- EXTERNAL CONTENT TABLE
-- Raw data fetched from external platforms
-- Depends on: connected_accounts
-- ============================================================================
CREATE TABLE external_content (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id uuid NOT NULL REFERENCES connected_accounts(id) ON DELETE CASCADE,

  content_type text NOT NULL,  -- 'github_repo', 'strava_activity', 'spotify_track'


  raw_data jsonb NOT NULL,  -- Full API response
  content_hash text,  -- md5(raw_data) for change detection (TODO maybe choose faster, better algo)

  fetched_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz,

  UNIQUE (account_id, content_hash)
);
