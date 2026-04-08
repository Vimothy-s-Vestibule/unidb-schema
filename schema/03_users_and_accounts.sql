-- ============================================================================
-- VESTIBULE USERS TABLE (canonical user entity)
-- A user can have 0 or many Discord accounts
-- ============================================================================

CREATE TABLE vestibule_users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  real_first text,
  real_last text,
  nickname text,

  intro_message_id bigint,  -- FK added via ALTER TABLE after messages exists

  -- Aggregated personality scores from all messages
  score_id uuid REFERENCES scores(id),
  score_last_updated timestamptz,

  -- Generated HEXACO diagram images
  current_diagram bytea,
  current_diagram_last_updated timestamptz,

  intro_diagram bytea
);

-- ============================================================================
-- DISCORD ACCOUNTS TABLE
-- Many-to-one: multiple discord accounts can belong to one vestibule_user
-- ============================================================================

CREATE TABLE discord_accounts (
  discord_user_id bigint PRIMARY KEY,
  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  username text NOT NULL UNIQUE,
  display_name text NOT NULL
);

-- Add FK from messages to discord_accounts (deferred due to dependency order)
ALTER TABLE messages 
  ADD CONSTRAINT fk_messages_sent_by 
  FOREIGN KEY (sent_by) REFERENCES discord_accounts(discord_user_id);
