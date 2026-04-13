-- ============================================================================
-- MESSAGES TABLE
-- Depends on: channels, scores
-- ============================================================================

CREATE TABLE messages (
  message_id bigint PRIMARY KEY,
  channel_id bigint NOT NULL REFERENCES channels(channel_id),

  sent_by bigint NOT NULL,

  content text NOT NULL,

  sent_at timestamptz NOT NULL,
  added_at timestamptz NOT NULL DEFAULT NOW(),
  last_edited timestamptz,
  deleted_at timestamptz,

  -- Self-reference for reply chains
  in_reply_to bigint REFERENCES messages(message_id),

  -- LLM personality score for only this message
  score_id uuid REFERENCES scores(id),

  -- Processing pipeline metadata
  triage_status text NOT NULL DEFAULT 'pending',    -- pending: Will be processed/processing: A worker is curretly processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
 -- |
 -- |
 -- ⌄
  is_significant boolean NOT NULL, -- Whether an LLM should score and extract personality from it, this field is also being set by an LLM TODO
 -- |
 -- |
 -- ⌄
  skill_status text, -- NULL: insignificant for skills/pending: Will be processed/processing: A worker is curretly processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
  personality_status text, -- NULL: insignificant for persinality extraction/pending: Will be processed/processing: A worker is curretly processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
  -- TODO make it so admins can manually override messages to be included/excluded from skills or personality processing
  processed_at timestamptz
);
