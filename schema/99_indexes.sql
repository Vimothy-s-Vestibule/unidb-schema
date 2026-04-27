-- Discord accounts
CREATE INDEX idx_discord_accounts_user ON discord_accounts(vestibule_user_id);

-- Message reactions
CREATE INDEX idx_message_reactions_message ON message_reactions(message_id);
CREATE INDEX idx_message_reactions_user ON message_reactions(user_id);

-- User presence: current status per user
CREATE INDEX idx_presence_current_status ON discord_user_presence(user_id)
  WHERE ended_at IS NULL AND status IS NOT NULL;
-- User presence: current activities per user
CREATE INDEX idx_presence_current_activity ON discord_user_presence(user_id)
  WHERE ended_at IS NULL AND activity_type IS NOT NULL;
-- User presence: enabling historical queries "who was online at time X"
CREATE INDEX idx_presence_history ON discord_user_presence(user_id, started_at, ended_at);
-- User presence: time-based queries
CREATE INDEX idx_presence_time ON discord_user_presence(started_at DESC);

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


-- Connected accounts
CREATE INDEX idx_connected_accounts_user_id ON connected_accounts(vestibule_user_id);

-- User activities
CREATE INDEX idx_user_facts_and_activities_user ON user_facts_and_activities(user_id);
CREATE INDEX idx_user_facts_and_activities_type ON user_facts_and_activities(user_id, type);
CREATE INDEX idx_user_facts_and_activities_started ON user_facts_and_activities(user_id, started_at DESC) WHERE record_type = 'activity';
CREATE INDEX idx_user_facts_and_activities_current ON user_facts_and_activities(user_id, is_current) WHERE record_type = 'fact';

-- Vector similarity search (HNSW index)
-- CREATE INDEX idx_scores_embedding ON scores
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;

-- CREATE INDEX idx_skills_embedding ON skills
--   USING hnsw (embedding vector_cosine_ops)
--   WHERE embedding IS NOT NULL;

-- YouTube tables
CREATE INDEX idx_youtube_comments_video ON youtube_comments(video_id);
CREATE INDEX idx_youtube_comments_author ON youtube_comments(author_channel_id);
CREATE INDEX idx_youtube_comments_published ON youtube_comments(published_at DESC);
