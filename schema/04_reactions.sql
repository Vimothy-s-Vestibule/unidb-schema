-- ============================================================================
-- MESSAGE REACTIONS TABLE
-- Tracks emoji reactions on Discord messages
-- Depends on: messages, discord_accounts
-- ============================================================================

CREATE TABLE message_reactions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  message_id bigint NOT NULL REFERENCES messages(message_id) ON DELETE CASCADE,
  user_id bigint NOT NULL REFERENCES discord_accounts(discord_user_id) ON DELETE CASCADE,

  -- Unicode emoji string or custom emoji snowflake ID
  emoji text NOT NULL,
  -- Display name (useful for custom emojis)
  emoji_name text,
  -- Whether this is a custom emoji (vs unicode)
  is_custom boolean NOT NULL DEFAULT false,
  -- Whether custom emoji is animated (GIF vs PNG) - only relevant when is_custom = true
  is_animated boolean NOT NULL DEFAULT false,
  -- CDN URL for custom emojis (NULL for unicode emojis)
  -- Format: https://cdn.discordapp.com/emojis/{id}.{png|gif}
  emoji_url text,

  reacted_at timestamptz NOT NULL DEFAULT NOW(),

  -- Prevent duplicate reactions (same user, same emoji, same message)
  UNIQUE (message_id, user_id, emoji)
);

-- Add FK from vestibule_users to messages (deferred due to circular dependency)
ALTER TABLE vestibule_users
  ADD CONSTRAINT fk_vestibule_users_intro_message
  FOREIGN KEY (intro_message_id) REFERENCES messages(message_id);
