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

CREATE TABLE message_edits (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id bigint NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
  old_content text NOT NULL,
  edited_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE message_attachments (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id bigint NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
  asset_id uuid NOT NULL REFERENCES media_assets(id) ON DELETE RESTRICT,
  
  added_at timestamptz NOT NULL DEFAULT NOW(),
  
  -- If the user edits the message and/or removes the attachment, we set deleted_at so we don't lose it for historical record
  deleted_at timestamptz
);
