-- Depends on: channels, scores
CREATE TABLE messages (
  message_id bigint PRIMARY KEY,
  channel_id bigint NOT NULL REFERENCES discord_channels(channel_id),

  sent_by bigint NOT NULL,

  content text NOT NULL,

  sent_at timestamptz NOT NULL,
  added_at timestamptz NOT NULL DEFAULT NOW(),
  last_edited timestamptz,
  deleted_at timestamptz,

  in_reply_to bigint REFERENCES messages(message_id)
);
