CREATE TABLE topic (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    -- LLM-extracted concise summary of what the topic messages discuss
    name text NOT NULL
);


CREATE TABLE topic_message_relation (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id uuid REFERENCES topic(id) NOT NULL,
    message_id bigint REFERENCES messages(message_id) NOT NULL
);
