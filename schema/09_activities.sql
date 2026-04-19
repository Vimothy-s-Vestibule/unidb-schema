-- LLM-extracted activities from messages and external content
-- Depends on: vestibule_users, external_content, messages
-- TODO make it so admins can manually add activities
CREATE TABLE user_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,

  -- What kind of activity: 'run', 'cycle', 'swim', 'git_commit', 'took_job', 'add_experience', 'linkedin_post', 'song_listen', 'podcast_listen', 'skill', 'project', '' etc.
  activity_type text NOT NULL,
  -- Human-readable summary: "10km marathon", "rust-analyzer contribution", "Took job as Senior Engineer at Google"
  label text NOT NULL,

  -- When the activity happened (not when extracted)
  occurred_at timestamptz,

  -- Sources (at least one required)
  external_content_id uuid REFERENCES external_content(id),
  message_id bigint REFERENCES messages(message_id),
  youtube_comment_id text REFERENCES youtube_comments(comment_id),
  reasoning text,

  CHECK (external_content_id IS NOT NULL OR message_id IS NOT NULL OR youtube_comment_id IS NOT NULL)
);
