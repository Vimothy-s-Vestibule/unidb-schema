-- This file should undo anything in `up.sql`
ALTER TABLE "messages" DROP COLUMN "sent_at";
ALTER TABLE "messages" DROP COLUMN "added_at";
ALTER TABLE "messages" DROP COLUMN "score_id";
ALTER TABLE "messages" ADD COLUMN "created_at" TIMESTAMPTZ NOT NULL;

ALTER TABLE "vestibule_users" DROP COLUMN "score_id";
ALTER TABLE "vestibule_users" ADD COLUMN "honesty_humility" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "emotionality" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "extraversion" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "agreeableness" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "conscientiousness" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "openness_to_experience" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "agency" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "communion" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "self_direction" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "stimulation" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "hedonism" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "achievement" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "power" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "security" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "conformity" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "tradition" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "benevolence" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "universalism" FLOAT8 NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "intro_embedding" VECTOR;
ALTER TABLE "vestibule_users" ADD COLUMN "intro_diagram" BYTEA;
ALTER TABLE "vestibule_users" ADD COLUMN "activities" TEXT[] NOT NULL;
ALTER TABLE "vestibule_users" ADD COLUMN "domains" TEXT[] NOT NULL;

DROP TABLE IF EXISTS "scores";
