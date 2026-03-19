-- Your SQL goes here

-- 1. Modify user_skills to have `id` as the only primary key
-- CASCADE will automatically drop any foreign keys in user_skill_evidence that rely on this PK
ALTER TABLE user_skills
DROP CONSTRAINT IF EXISTS user_skills_pkey CASCADE;
-- Add the new column and set the new PK
ALTER TABLE user_skills 
ADD COLUMN id UUID NOT NULL DEFAULT gen_random_uuid(); -- Ensure it has a default for existing rows
ALTER TABLE user_skills
ADD PRIMARY KEY (id);
-- 2. Modify user_skill_evidence to reference user_skills(id)
ALTER TABLE user_skill_evidence
DROP CONSTRAINT IF EXISTS user_skill_evidence_pkey CASCADE;
ALTER TABLE user_skill_evidence
ADD COLUMN user_skill_id UUID NOT NULL;
-- 3. Drop the old composite columns from the evidence table
ALTER TABLE user_skill_evidence
DROP COLUMN user_id,
DROP COLUMN skill_id;
-- 4. Set the new primary key and foreign keys
-- 4. Set the new primary key and foreign keys
ALTER TABLE user_skill_evidence
ADD PRIMARY KEY (user_skill_id, message_id),
ADD CONSTRAINT user_skill_evidence_user_skill_id_fkey 
    FOREIGN KEY (user_skill_id) REFERENCES user_skills(id) ON DELETE CASCADE;