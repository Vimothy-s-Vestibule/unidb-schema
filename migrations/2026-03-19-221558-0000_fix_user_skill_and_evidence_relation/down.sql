-- This file should undo anything in `up.sql`

-- 1. Drop the new foreign key and primary key from user_skill_evidence
ALTER TABLE user_skill_evidence
DROP CONSTRAINT IF EXISTS user_skill_evidence_user_skill_id_fkey CASCADE,
DROP CONSTRAINT IF EXISTS user_skill_evidence_pkey CASCADE;
-- 2. Restore the old columns to user_skill_evidence and drop the new one
-- Note: 'user_id' and 'skill_id' will be added empty. If data preservation was needed
-- on rollback, we would need to join with `user_skills` before dropping `user_skill_id`.
-- For structural rollback, this matches the destructive nature of up.sql.
ALTER TABLE user_skill_evidence
DROP COLUMN user_skill_id,
ADD COLUMN user_id TEXT NOT NULL DEFAULT '',
ADD COLUMN skill_id UUID NOT NULL DEFAULT gen_random_uuid();
-- Remove defaults after creation to enforce strict constraints
ALTER TABLE user_skill_evidence 
ALTER COLUMN user_id DROP DEFAULT,
ALTER COLUMN skill_id DROP DEFAULT;
-- 3. Restore the old composite primary key and foreign keys on evidence
ALTER TABLE user_skill_evidence
ADD PRIMARY KEY (user_id, skill_id, message_id),
ADD CONSTRAINT user_skill_evidence_user_id_fkey 
    FOREIGN KEY (user_id) REFERENCES vestibule_users(discord_user_id),
ADD CONSTRAINT user_skill_evidence_skill_id_fkey 
    FOREIGN KEY (skill_id) REFERENCES skills(id);
-- 4. Revert user_skills (Drop the new UUID primary key)
ALTER TABLE user_skills
DROP CONSTRAINT IF EXISTS user_skills_pkey CASCADE;
ALTER TABLE user_skills
DROP COLUMN id;
-- 5. Restore the old composite primary key on user_skills
ALTER TABLE user_skills
ADD PRIMARY KEY (user_id, skill_id);