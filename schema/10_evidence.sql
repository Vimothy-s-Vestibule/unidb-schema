-- Depends on: user_facts_and_activities, messages, youtube_comments, external_content, discord_user_presence

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

-- FUNCTION: Check if a fact/activity has at least one piece of evidence
CREATE OR REPLACE FUNCTION verify_fact_has_evidence()
RETURNS trigger AS $$
BEGIN
  -- If it's a manual entry, we might not require LLM evidence, but if we want to enforce it globally:
  IF NOT EXISTS (SELECT 1 FROM fact_and_activity_evidence WHERE fact_or_activity_id = NEW.id) THEN
    RAISE EXCEPTION 'A fact or activity must have at least one supporting evidence record inserted. (Fact ID: %)', NEW.id;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- TRIGGER: Runs at the END of the transaction (DEFERRABLE INITIALLY DEFERRED)
-- This allows the app to INSERT the fact, then INSERT the evidence, and only then does Postgres check.
CREATE CONSTRAINT TRIGGER ensure_fact_has_evidence
AFTER INSERT ON user_facts_and_activities
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION verify_fact_has_evidence();
