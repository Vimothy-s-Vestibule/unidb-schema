-- Depends on: messages, discord_accounts
CREATE TABLE discord_emojis (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  -- Unicode emoji string or custom emoji snowflake ID
  discord_emoji_id text NOT NULL,

  -- Some string if the emoji is a custom one created by some server, otherwise NULL
  from_guild text,

  -- Display name (useful for custom emojis)
  emoji_display_name text,

  -- Whether custom emoji is animated (GIF vs PNG) - only relevant when from_guild is not NULL
  is_animated boolean NOT NULL DEFAULT false,

  -- CDN URL for custom emojis (NULL for non-custom emojis)
  -- Format: TODO
  emoji_url text,

  asset_id uuid REFERENCES media_assets(id)
);

CREATE TABLE message_reactions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id bigint NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id),

  emoji_id uuid REFERENCES discord_emojis(id) ON DELETE RESTRICT,

  reacted_at timestamptz NOT NULL DEFAULT NOW(),

  -- Prevent duplicate reactions
  UNIQUE (message_id, user_id, emoji_id)
);
