CREATE TABLE social_platforms (
  id uuid PRIMARY KEY,
  platform_name text NOT NULL UNIQUE,
  -- URL: https://strava.com https://spotify.com https://linkedin.com ...
  homepage text NOT NULL,
  access_type platform_access NOT NULL,
  logo uuid REFERENCES media_assets(id)
);

-- Seed initial social platforms, more can be added anytime
INSERT INTO social_platforms (id, platform_name, homepage, access_type)
VALUES
  (gen_random_uuid(), 'Strava', 'https://strava.com', 'oauth_required'),
  (gen_random_uuid(), 'Spotify', 'https://spotify.com', 'oauth_required'),
  (gen_random_uuid(), 'LinkedIn', 'https://linkedin.com', 'unavailable'),
  (gen_random_uuid(), 'YouTube', 'https://youtube.com', 'public'),
  -- TODO curl -L -X GET "https://api.github.com/users/alex" -H "Accept: application/vnd.github+json"
  (gen_random_uuid(), 'GitHub', 'https://github.com', 'public'),
  -- TODO Use Nvidia parakeet to transscribe Sylvan's daily internal monologue and run analysis on it for maximum data extraction: https://vimothee.substack.com/feed
  (gen_random_uuid(), 'Substack', 'https://substack.com', 'public')
ON CONFLICT (platform_name) DO NOTHING;

-- TODO seed db with vestibule channel, reans, big smoke, tokitsuno and all others who have their youtube connected to their discord or "watch my latest video:" in their discord bio
-- Depends on: vestibule_users, platforms, messages
CREATE TABLE connected_accounts (
  -- Vestibule-specific internal UUID
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  -- With which user the account is associated
  vestibule_user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  -- UUID of the platform, see social_platforms
  platform_id uuid NOT NULL REFERENCES social_platforms(id),

  -- For YT: The Channel ID
  -- Might be NULL for platforms like GitHub
  platform_user_id text,

  -- For YT: Sylvan Franklin
  platform_display_name text NOT NULL,

  -- For YT: @SylvanFranklin
  platform_username text NOT NULL,

  bio text,
  -- On Youtube, for example, some people fill in where they are located in the world or some toher stuff
  bio_additional_info text,
  
  profile_picture uuid REFERENCES media_assets(id),


  -- For finding similar users in Discord accounts and across other platforms, TODO choose universal vector dimensions amount and choose embedding model (preferably selfhostable)
  -- TODO Embed both username and displayname to create 2 "maps" of these to see what matches
  name_embedding vector,


  -- Metadata about how we found out
  mention_message_id bigint REFERENCES messages(message_id),
  -- Or if linked manually, the reason why we believe the account is linked to the discord user
  reasoning text NOT NULL,



  UNIQUE (vestibule_user_id, platform_id, platform_username),
  CHECK (mention_message_id IS NOT NULL OR reasoning IS NOT NULL)
);
