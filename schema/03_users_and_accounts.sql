-- ============================================================================
-- VESTIBULE USERS TABLE (canonical user entity)
-- A user can have 0 or many Discord accounts
-- ============================================================================

CREATE TABLE vestibule_users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  real_first text,
  real_last text,
  nickname text,

  -- FK added via ALTER TABLE after messages exists, It can be null if we add users from outside of the discord and because writing an intro is not mandatory anymore
  intro_message_id bigint,  

  -- Aggregated personality scores from all messages
  score_id uuid REFERENCES scores(id),
  score_last_updated timestamptz,

  -- Generated HEXACO diagram images
  current_diagram bytea,
  current_diagram_last_updated timestamptz,

  intro_diagram bytea
);


-- Multiple discord accounts can belong to one vestibule_user
-- TODO Save Discord connections such as LinkedIn, Spotify, Twitch, and others for big metadata gathering
CREATE TABLE discord_accounts (
  discord_user_id bigint PRIMARY KEY,
  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  username text NOT NULL UNIQUE,
  display_name text NOT NULL
);

-- FK from messages to discord_accounts (deferred due to dependency order)
ALTER TABLE messages 
  ADD CONSTRAINT fk_messages_sent_by 
  FOREIGN KEY (sent_by) REFERENCES discord_accounts(discord_user_id);

-- FK from vestibule_users to messages (deferred due to dependency order)
ALTER TABLE vestibule_users
  ADD CONSTRAINT fk_vestibule_users_intro_message
  FOREIGN KEY (intro_message_id) REFERENCES messages(message_id);
