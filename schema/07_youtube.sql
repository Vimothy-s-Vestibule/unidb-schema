-- Youtube related tables
-- Depends on: scores, vestibule_users


CREATE TABLE youtube_videos (
  -- The part after youtube.com/watch?v=
  video_id text PRIMARY KEY,

  channel_id uuid NOT NULL REFERENCES connected_accounts(id) ON DELETE CASCADE,
  title text NOT NULL,
  description text,
  published_at timestamptz NOT NULL,
  total_views bigint NOT NULL DEFAULT 0,
  total_likes bigint NOT NULL DEFAULT 0,
  total_comments bigint NOT NULL DEFAULT 0,
  
  -- Content Details
  duration_seconds integer NOT NULL,
  broadcast_status text NOT NULL, -- Values: 'video', 'current_live', 'past_live', 'scheduled_live', 'none'
  
  -- Media & Discovery
  thumbnail_asset_id uuid REFERENCES media_assets(id),
  transcript_asset_id uuid REFERENCES media_assets(id),
  -- Downloaded audio ID
  audio_asset_id uuid REFERENCES media_assets(id),
  keyword_tags text[],

  added_at timestamptz DEFAULT NOW()
);

CREATE TABLE youtube_comments (
  comment_id text PRIMARY KEY,
  video_id text NOT NULL REFERENCES youtube_videos(video_id) ON DELETE CASCADE,
  
  -- The channel that posted the comment
  author_raw_channel_id text NOT NULL,
  -- If the channel is already in the db, populate this field too, TODO When and how do we decide If we want to add a user to the db, I'd say after the second comment they wrute or if they are deemed significant and can be linked to a vestibule user by an admin.
  author_channel_id uuid REFERENCES connected_accounts(id) ON DELETE CASCADE,

  content text NOT NULL,
  
  like_count integer NOT NULL,
  
  published_at timestamptz NOT NULL,
  updated_at timestamptz,
  
  -- Self-reference for replies
  in_reply_to text REFERENCES youtube_comments(comment_id),
  
  triage_status text NOT NULL DEFAULT 'pending',
  is_significant boolean NOT NULL,
  skill_status text,
  personality_status text,
  processed_at timestamptz
);
