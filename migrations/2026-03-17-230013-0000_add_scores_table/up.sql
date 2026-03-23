-- Your SQL goes here
CREATE TABLE "scores"(
	"score_id" TEXT NOT NULL PRIMARY KEY,
	"honesty_humility" FLOAT8 NOT NULL,
	"emotionality" FLOAT8 NOT NULL,
	"extraversion" FLOAT8 NOT NULL,
	"agreeableness" FLOAT8 NOT NULL,
	"conscientiousness" FLOAT8 NOT NULL,
	"openness_to_experience" FLOAT8 NOT NULL,
	"agency" FLOAT8 NOT NULL,
	"communion" FLOAT8 NOT NULL,
	"self_direction" FLOAT8 NOT NULL,
	"stimulation" FLOAT8 NOT NULL,
	"hedonism" FLOAT8 NOT NULL,
	"achievement" FLOAT8 NOT NULL,
	"power" FLOAT8 NOT NULL,
	"security" FLOAT8 NOT NULL,
	"conformity" FLOAT8 NOT NULL,
	"tradition" FLOAT8 NOT NULL,
	"benevolence" FLOAT8 NOT NULL,
	"universalism" FLOAT8 NOT NULL,
	"activities" TEXT[] NOT NULL,
	"domains" TEXT[] NOT NULL,
	"embedding" VECTOR,
	"intro_diagram" BYTEA,
	"current_diagram" BYTEA
);

ALTER TABLE "messages" DROP COLUMN "created_at";
ALTER TABLE "messages" ADD COLUMN "sent_at" TIMESTAMPTZ NOT NULL;
ALTER TABLE "messages" ADD COLUMN "added_at" TIMESTAMPTZ NOT NULL;
ALTER TABLE "messages" ADD COLUMN "score_id" TEXT REFERENCES "scores"("score_id") ON DELETE CASCADE;

ALTER TABLE "vestibule_users" DROP COLUMN "honesty_humility";
ALTER TABLE "vestibule_users" DROP COLUMN "emotionality";
ALTER TABLE "vestibule_users" DROP COLUMN "extraversion";
ALTER TABLE "vestibule_users" DROP COLUMN "agreeableness";
ALTER TABLE "vestibule_users" DROP COLUMN "conscientiousness";
ALTER TABLE "vestibule_users" DROP COLUMN "openness_to_experience";
ALTER TABLE "vestibule_users" DROP COLUMN "agency";
ALTER TABLE "vestibule_users" DROP COLUMN "communion";
ALTER TABLE "vestibule_users" DROP COLUMN "self_direction";
ALTER TABLE "vestibule_users" DROP COLUMN "stimulation";
ALTER TABLE "vestibule_users" DROP COLUMN "hedonism";
ALTER TABLE "vestibule_users" DROP COLUMN "achievement";
ALTER TABLE "vestibule_users" DROP COLUMN "power";
ALTER TABLE "vestibule_users" DROP COLUMN "security";
ALTER TABLE "vestibule_users" DROP COLUMN "conformity";
ALTER TABLE "vestibule_users" DROP COLUMN "tradition";
ALTER TABLE "vestibule_users" DROP COLUMN "benevolence";
ALTER TABLE "vestibule_users" DROP COLUMN "universalism";
ALTER TABLE "vestibule_users" DROP COLUMN "intro_embedding";
ALTER TABLE "vestibule_users" DROP COLUMN "intro_diagram";
ALTER TABLE "vestibule_users" DROP COLUMN "activities";
ALTER TABLE "vestibule_users" DROP COLUMN "domains";
ALTER TABLE "vestibule_users" ADD COLUMN "score_id" TEXT REFERENCES "scores"("score_id") ON DELETE CASCADE;

