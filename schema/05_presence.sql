-- Depends on: discord_accounts, messages
CREATE TABLE discord_user_presence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  -- online, forced_online, absent, do_not_disturb, offline
  status text,

  -- NULL when no activity is currently happening
  activity_type text,

  -- Activity details (only for presence_type = 'activity', matching serenity's Activity struct)
  name text,
  details text,
  state text,
  url text,

  started_at timestamptz NOT NULL DEFAULT NOW(),
  ended_at timestamptz,       -- NULL = current/still active

  -- If status = 'forced_online', we record the message that proved they were online
  evidence_message_id bigint REFERENCES messages(message_id) ON DELETE CASCADE,

  -- evidence_message_id is only allowed when status is 'forced_online'
  CHECK (
    evidence_message_id IS NULL OR status = 'forced_online'
  ),

  CONSTRAINT valid_presence_range CHECK (ended_at IS NULL OR ended_at > started_at)
);
