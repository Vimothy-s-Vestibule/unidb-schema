-- USER SKILLS TABLE
-- Depends on: vestibule_users, skills

CREATE TABLE user_skills (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,
  skill_id uuid NOT NULL REFERENCES skills(id),

  -- Skill level 0-10
  level smallint NOT NULL CHECK (level >= 0 AND level <= 10),

  -- LLM reasoning context
  llm_context text,

  -- Prevent duplicate user-skill combinations
  UNIQUE (user_id, skill_id)
);


-- Depends on: user_skills, messages
CREATE TABLE user_skill_evidence (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_skill_id uuid NOT NULL REFERENCES user_skills(id),
  message_id bigint REFERENCES messages(message_id), -- evidence may also be a Strava activity, for example proving the user can run a certain pace
  youtube_comment_id text REFERENCES youtube_comments(comment_id),

  -- How strongly this message supports the skill assessment
  weight real NOT NULL,
  reasoning text NOT NULL,

  CHECK (message_id IS NOT NULL OR youtube_comment_id IS NOT NULL),
  UNIQUE (user_skill_id, message_id),
  UNIQUE (user_skill_id, youtube_comment_id)
);
