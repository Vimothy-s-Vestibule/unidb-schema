-- Depends on: discord_accounts
CREATE TABLE user_presence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  -- Status: online, forced_online, absent, do_not_disturb, offline
  status text NOT NULL,

  started_at timestamptz NOT NULL,
  ended_at timestamptz,  -- NULL = current status

  CONSTRAINT valid_presence_range CHECK (ended_at IS NULL OR ended_at > started_at)
);


-- Links messages sent while user showed as offline/invisible
CREATE TABLE forced_online_evidence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,
  message_id bigint NOT NULL UNIQUE REFERENCES messages(message_id) ON DELETE CASCADE,
  presence_id uuid NOT NULL REFERENCES user_presence(id),

  -- Link to the presence record at that time

  detected_at timestamptz NOT NULL DEFAULT NOW()
);
