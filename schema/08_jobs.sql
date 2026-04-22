CREATE TABLE jobs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  
  -- Job metadata
  job_type text NOT NULL, -- 'youtube_channel_scrape', 'discord_channel_scrape', 'strava_sync', etc.
  status text NOT NULL DEFAULT 'pending', -- 'pending', 'in_progress', 'completed', 'failed'

  -- Worker tracking, set both to NULL after
  locked_by_worker_id text, -- ID of the worker currently processing this
  locked_at timestamptz,

  -- Scheduling and retries
  scheduled_for timestamptz NOT NULL DEFAULT NOW(),
  attempts int NOT NULL DEFAULT 0,
  max_attempts int NOT NULL DEFAULT 3,
  last_error text,

  -- POLYMORPHIC FOREIGN KEYS 
  -- (Only ONE of these should be NOT NULL for a given row)
  youtube_video_id text REFERENCES youtube_videos(video_id) ON DELETE CASCADE,
  discord_channel_id bigint REFERENCES discord_channels(channel_id) ON DELETE CASCADE,
  connected_account_id uuid REFERENCES connected_accounts(id) ON DELETE CASCADE,
  media_asset_id uuid REFERENCES media_assets(id) ON DELETE CASCADE,

  -- Check constraint to ensure exactly one target table is specified
  CONSTRAINT has_single_target CHECK (
    (youtube_video_id IS NOT NULL)::int + 
    (discord_channel_id IS NOT NULL)::int + 
    (connected_account_id IS NOT NULL)::int +
    (media_asset_id IS NOT NULL)::int = 1
  )
);
