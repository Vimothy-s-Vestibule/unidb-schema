CREATE TABLE topic (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- LLM-extracted concise summary of what the topic messages discuss.
    -- Unique: topic-sorter reuses an existing topic of the same name.
    name text NOT NULL UNIQUE,
    embedding vector
);


CREATE TABLE topic_message_relation (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id uuid REFERENCES topic(id) NOT NULL,
    -- Unique: topic-sorter assigns at most one topic per message (relax to
    -- UNIQUE (topic_id, message_id) if messages may ever get several topics).
    message_id bigint REFERENCES messages(message_id) NOT NULL UNIQUE
);

-- Tracks which messages the topic-sorter has attempted to classify.
-- Presence of a row = processed; no row = not yet tried.
-- A message with no topic_message_relation row was processed but assigned no topic.
CREATE TABLE message_classification_attempts (
    message_id bigint PRIMARY KEY REFERENCES messages(message_id),
    attempted_at timestamptz NOT NULL DEFAULT NOW()
);
