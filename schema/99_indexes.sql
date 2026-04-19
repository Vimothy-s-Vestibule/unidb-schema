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

-- User skills
CREATE INDEX idx_user_skills_user_id ON user_skills(user_id);

-- Connected accounts
CREATE INDEX idx_connected_accounts_user_id ON connected_accounts(vestibule_user_id);

-- User activities
CREATE INDEX idx_user_activities_user ON user_activities(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_user_activities_discord_user ON user_activities(discord_user_id) WHERE discord_user_id IS NOT NULL;
CREATE INDEX idx_user_activities_type ON user_activities(activity_type);
CREATE INDEX idx_user_activities_source ON user_activities(source);
CREATE INDEX idx_user_activities_occurred ON user_activities(started_at DESC) WHERE started_at IS NOT NULL;
-- Current ongoing activities (discord presence)
CREATE INDEX idx_user_activities_current ON user_activities(discord_user_id) WHERE ended_at IS NULL AND source = 'discord_presence';

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
CREATE INDEX idx_youtube_videos_channel ON youtube_videos(channel_id);
