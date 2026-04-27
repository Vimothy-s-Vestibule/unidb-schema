-- Depends on: user_facts_and_activities, messages, youtube_comments, external_content, discord_user_presence

-- Raw data fetched from external platforms
-- Depends on: connected_accounts
CREATE TABLE external_content (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id uuid NOT NULL REFERENCES connected_accounts(id) ON DELETE CASCADE,

  content_type text NOT NULL,  -- 'github_repo', 'strava_activity', 'spotify_track' (NOT youtube — use youtube_videos/youtube_comments tables instead)


  raw_data jsonb NOT NULL,  -- Full API response
  content_hash text,  -- md5(raw_data) for change detection (TODO maybe choose faster, better algo)

  fetched_at timestamptz NOT NULL DEFAULT NOW(),
  -- If it was fetched multiple times, when it was fetched the last time
  updated_at timestamptz,

  UNIQUE (account_id, content_hash),
  -- YouTube has dedicated tables: youtube_videos, youtube_comments
  CHECK (content_type NOT IN ('youtube_video', 'youtube_comment', 'youtube_channel'))
);

CREATE TABLE fact_and_activity_evidence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  
  fact_or_activity_id uuid NOT NULL REFERENCES user_facts_and_activities(id) ON DELETE CASCADE,
  
  -- The source of the evidence (Polymorphic FKs)
  message_id bigint REFERENCES messages(message_id) ON DELETE CASCADE,
  youtube_comment_id text REFERENCES youtube_comments(comment_id) ON DELETE CASCADE,
  external_content_id uuid REFERENCES external_content(id) ON DELETE CASCADE,
  discord_presence_id uuid REFERENCES discord_user_presence(id) ON DELETE CASCADE,

  -- How strongly this evidence supports the fact or activity (0.0 - 1.0)
  weight real NOT NULL DEFAULT 1.0,
  
  -- The LLM's explanation of why this specific source supports the fact/activity
  reasoning text NOT NULL,

  -- Ensure exactly one source is provided per evidence row
  CONSTRAINT has_single_evidence_source CHECK (
    (message_id IS NOT NULL)::int + 
    (youtube_comment_id IS NOT NULL)::int + 
    (external_content_id IS NOT NULL)::int +
    (discord_presence_id IS NOT NULL)::int = 1
  ),

  -- Prevent duplicate evidence for the exact same fact/source combo
  UNIQUE NULLS NOT DISTINCT (fact_or_activity_id, message_id, youtube_comment_id, external_content_id, discord_presence_id)
);


CREATE OR REPLACE FUNCTION verify_fact_has_evidence()
RETURNS trigger AS $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM fact_and_activity_evidence WHERE fact_or_activity_id = NEW.id) THEN
    RAISE EXCEPTION 'A fact or activity must have at least one supporting evidence record already inserted. (Fact/Activity ID: %)', NEW.id;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- INSERT the fact, then INSERT the evidence, and only then does Postgres check.
CREATE CONSTRAINT TRIGGER ensure_fact_has_evidence
AFTER INSERT ON user_facts_and_activities
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION verify_fact_has_evidence();
