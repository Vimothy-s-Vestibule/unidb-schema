-- ============================================================================
-- INDEXES
-- ============================================================================

-- Discord accounts
CREATE INDEX idx_discord_accounts_user ON discord_accounts(vestibule_user_id);

-- Message reactions
CREATE INDEX idx_message_reactions_message ON message_reactions(message_id);
CREATE INDEX idx_message_reactions_user ON message_reactions(user_id);

-- User presence: current status per user
CREATE INDEX idx_presence_current ON user_presence(user_id) WHERE ended_at IS NULL;
-- User presence: historical queries "who was online at time X"
CREATE INDEX idx_presence_history ON user_presence(user_id, started_at, ended_at);
-- User presence: time-based queries
CREATE INDEX idx_presence_time ON user_presence(started_at DESC);

-- User presence activities: current activities per user
CREATE INDEX idx_presence_activities_current ON user_presence_activities(user_id) WHERE ended_at IS NULL;
-- User presence activities: historical queries
CREATE INDEX idx_presence_activities_history ON user_presence_activities(user_id, started_at DESC);

-- Forced online evidence
CREATE INDEX idx_forced_online_user ON forced_online_evidence(user_id, detected_at DESC);

-- Messages: common queries
CREATE INDEX idx_messages_sent_by ON messages(sent_by);
CREATE INDEX idx_messages_channel_id ON messages(channel_id);
CREATE INDEX idx_messages_sent_at ON messages(sent_at DESC);
CREATE INDEX idx_messages_user_sent ON messages(sent_by, sent_at DESC);
CREATE INDEX idx_messages_channel_sent ON messages(channel_id, sent_at DESC);
CREATE INDEX idx_messages_in_reply_to ON messages(in_reply_to) WHERE in_reply_to IS NOT NULL;

-- Partial index for non-deleted messages
CREATE INDEX idx_messages_active ON messages(channel_id, sent_at DESC)
  WHERE deleted_at IS NULL;

-- Message processing pipeline
CREATE INDEX idx_messages_triage_pending ON messages(added_at)
  WHERE triage_status = 'pending';
CREATE INDEX idx_messages_skill_pending ON messages(added_at)
  WHERE skill_status = 'pending';
CREATE INDEX idx_messages_personality_pending ON messages(added_at)
  WHERE personality_status = 'pending';

-- User skills
CREATE INDEX idx_user_skills_user_id ON user_skills(user_id);

-- Platform associations and syncing
CREATE INDEX idx_user_platform_association_user_id ON user_platform_association(vestibule_user_id);
CREATE INDEX idx_user_platform_sync_due ON user_platform_association(next_sync_at)
  WHERE sync_status = 'idle';

-- User activities
CREATE INDEX idx_user_activities_user ON user_activities(user_id);
CREATE INDEX idx_user_activities_type ON user_activities(user_id, activity_type);
CREATE INDEX idx_user_activities_occurred ON user_activities(user_id, occurred_at DESC);

-- Vector similarity search (HNSW index)
-- CREATE INDEX idx_scores_embedding ON scores
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;

-- CREATE INDEX idx_skills_embedding ON skills
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;
