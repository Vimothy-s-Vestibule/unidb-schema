-- Remove user presence tracking tables

-- Drop indexes first
DROP INDEX IF EXISTS idx_forced_online_user;
DROP INDEX IF EXISTS idx_presence_activities_history;
DROP INDEX IF EXISTS idx_presence_activities_current;
DROP INDEX IF EXISTS idx_presence_time;
DROP INDEX IF EXISTS idx_presence_history;
DROP INDEX IF EXISTS idx_presence_current;

-- Drop tables (order matters due to foreign keys)
DROP TABLE IF EXISTS forced_online_evidence;
DROP TABLE IF EXISTS user_presence_activities;
DROP TABLE IF EXISTS user_presence;
