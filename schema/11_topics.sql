CREATE TABLE topic (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- LLM-extracted concise summary of what the topic messages discuss
    name text NOT NULL,
    embedding vector
);


CREATE TABLE topic_message_relation (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id uuid REFERENCES topic(id) NOT NULL,
    message_id bigint REFERENCES messages(message_id) NOT NULL
);

-- Tracks which messages the topic-sorter has attempted to classify.
-- Presence of a row = processed; no row = not yet tried.
-- A message with no topic_message_relation row was processed but assigned no topic.
CREATE TABLE message_classification_attempts (
    message_id bigint PRIMARY KEY REFERENCES messages(message_id),
    attempted_at timestamptz NOT NULL DEFAULT NOW()
);
