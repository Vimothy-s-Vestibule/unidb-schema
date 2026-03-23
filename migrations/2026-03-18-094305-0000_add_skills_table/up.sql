-- Your SQL goes here

CREATE TABLE "skills" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- The exact canonical skill name (e.g., "Node.js", "Rust", "Photography")
    "name" TEXT NOT NULL UNIQUE, 
    
    -- Enables finding users with similar but differently named skills. TODO specify dimensions
    "embedding" VECTOR(1536)
);

-- Fast exact-match lookups during the LLM canonicalization phase
CREATE INDEX "idx_skills_name" ON "skills" ("name");

-- Links a user to a skill, storing their 0-10 level and historical context.
CREATE TABLE "user_skills" (
    "user_id" TEXT NOT NULL REFERENCES "vestibule_users"("discord_user_id") ON DELETE CASCADE,
    "skill_id" UUID NOT NULL REFERENCES "skills"("id") ON DELETE CASCADE,
    
    -- The currently inferred proficiency level (0-10)
    "level" SMALLINT NOT NULL CHECK ("level" >= 0 AND "level" <= 10),
    
    -- The rolling historical context/summary from the LLM (appended over time)
    "llm_context" TEXT,
    
    PRIMARY KEY ("user_id", "skill_id")
);

-- Index for finding all users with a specific skill quickly
CREATE INDEX "idx_user_skills_skill_id" ON "user_skills" ("skill_id");

-- Evidence that "proves" the users proficiency by linking the message ids of the messages where the user mentioned that they have the skill
CREATE TABLE "user_skill_evidence" (
    "user_id" TEXT NOT NULL,
    "skill_id" UUID NOT NULL,
    
    -- Link to the specific message that provides the evidence
    "message_id" TEXT NOT NULL REFERENCES "messages"("message_id") ON DELETE CASCADE,
    
    -- How important this message is in determining the overall proficiency of this user in this skill
    "weight" REAL NOT NULL,
    
    -- The LLM's justification for why the messages the user sent demonstrates the skill
    "reasoning" TEXT NOT NULL,
    
    -- Ensure a message is only used as evidence once per specific user-skill combination
    PRIMARY KEY ("user_id", "skill_id", "message_id"),
    
    -- Composite foreign key tying this back to the user's specific skill assessment
    FOREIGN KEY ("user_id", "skill_id") REFERENCES "user_skills"("user_id", "skill_id") ON DELETE CASCADE
);

-- Index to quickly find all evidence for a specific user's skill
CREATE INDEX "idx_user_skill_evidence_mapping" ON "user_skill_evidence" ("user_id", "skill_id");
-- Index to quickly find which skills a specific message contributed to
CREATE INDEX "idx_user_skill_evidence_message" ON "user_skill_evidence" ("message_id");
