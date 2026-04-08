-- ============================================================================
-- USER PLATFORM ASSOCIATION TABLE
-- Depends on: vestibule_users, messages, social_platforms
-- ============================================================================

CREATE TABLE user_platform_association (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  platform_id uuid NOT NULL REFERENCES social_platforms(id),

  platform_username text NOT NULL,
  platform_display_name text NOT NULL,
  profile_url text,

  -- Message where this association was mentioned/discovered
  platform_association_mention_message_id bigint REFERENCES messages(message_id),
  -- Reasoning for linking this user to this platform, TODO can be overriden to manually add associations by admins
  reasoning text NOT NULL,

  -- Sync scheduling
  last_synced_at timestamptz,
  last_sync_error text,
  sync_status text DEFAULT 'idle',         -- idle/running/failed

  -- Prevent duplicate user-platform combinations
  UNIQUE (vestibule_user_id, platform_id),
  CHECK (platform_association_mention_message_id IS NOT NULL OR reasoning IS NOT NULL)
);

-- ============================================================================
-- EXTERNAL CONTENT TABLE
-- Raw data fetched from external platforms (upserted on external_id)
-- Depends on: user_platform_association
-- ============================================================================
CREATE TABLE external_content (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  -- This is also the reason for fetching
  platform_association_id uuid NOT NULL REFERENCES user_platform_association(id),

  content_type text NOT NULL,              -- 'github_repo', 'strava_activity', 'linkedin_post', 'spotify_track'
  external_id text NOT NULL,               -- Platform's unique ID for this content

  raw_data jsonb NOT NULL,                 -- Full API response
  content_hash text,                       -- md5(raw_data) for change detection

  fetched_at timestamptz DEFAULT NOW(),
  updated_at timestamptz DEFAULT NOW(),

  UNIQUE (platform_association_id, content_type, external_id)
);
