-- Depends on: vestibule_users, external_content, messages, youtube_comments
-- TODO make it so admins can manually add activities
CREATE TABLE user_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,

  -- Where this activity came from
  -- 'llm_extraction': LLM-extracted from a message or content
  -- 'external_content': Obtained from the external_content table
  -- 'manual': manually added by an admin
  source text NOT NULL,

  -- What kind of activity: 'run', 'cycle', 'swim', 'git_commit', 'took_job', 'add_experience', 'linkedin_post', 'song_listen', 'podcast_listen', 'skill', 'project', etc.
  activity_type text NOT NULL,
  -- Human-readable summary: "10km marathon", "rust-analyzer contribution", "Took job as Senior Engineer at Google"
  label text NOT NULL,
  
  -- When the activity happened/started (not when extracted)
  started_at timestamptz,
  -- When the activity ended (NULL for point-in-time activities or still ongoing)
  ended_at timestamptz,

  -- Sources (at least one required when source = 'llm_extraction')
  external_content_id uuid REFERENCES external_content(id),
  message_id bigint REFERENCES messages(message_id),
  youtube_comment_id text REFERENCES youtube_comments(comment_id),
  discord_presence_id uuid REFERENCES discord_user_presence(id),
  -- And/or
  reasoning text,

  -- LLM-extracted activities must have at least one source
  CHECK (
    source != 'llm_extraction' OR
    (external_content_id IS NOT NULL OR message_id IS NOT NULL OR youtube_comment_id IS NOT NULL OR discord_presence_id IS NOT NULL)
  )
);
