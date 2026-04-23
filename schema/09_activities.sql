-- Depends on: vestibule_users, external_content, messages, youtube_comments
CREATE TABLE user_facts_and_activities (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES vestibule_users(id) ON DELETE CASCADE,

  -- Is this an 'activity' (has timeline/duration), a 'fact' (stateful knowledge), or a 'skill'
  record_type text NOT NULL CHECK (record_type IN ('activity', 'fact', 'skill')),

  -- Where this came from: 'llm_extraction', 'external_content', 'manual'
  source text NOT NULL,

  -- Type: 'took_job', 'timezone', 'location', 'hardware', 'editor', OR for skills: the skill name like 'Rust', 'Guitar'
  type text NOT NULL,
  
  -- Value: "10km marathon", "UTC+2", "Neovim", "Senior Software Engineer @ Google".
  value text NOT NULL,

  -- Confidence that the fact is actually true or activity is accurate and was interpreted correctly by the LLM
  confidence real NOT NULL DEFAULT 1.0,

  -- For 'skill' only: Proficiency level (0-10)
  level smallint CHECK (level >= 0 AND level <= 10),

  -- For 'activity' only: When it happened
  started_at timestamptz,
  ended_at timestamptz,

  -- For 'fact' or 'skill' only: Is this still true?
  is_current boolean DEFAULT true,

  created_at timestamptz NOT NULL DEFAULT NOW(),

  -- The embedding of a combination of the type and value of the fact/skill, useful to see e.g.:  Person 1 took a Junior swe position at google while person 2 interned there aswell
  type_value_embedding vector,
  
  -- Enforce field presence based on record_type
  CHECK (
    (record_type = 'activity' AND is_current IS NULL AND level IS NULL) OR
    (record_type = 'fact' AND started_at IS NULL AND ended_at IS NULL AND level IS NULL AND is_current IS NOT NULL) OR
    (record_type = 'skill' AND started_at IS NULL AND ended_at IS NULL AND level IS NOT NULL AND is_current IS NOT NULL AND level IS NOT NULL)
  )
);
