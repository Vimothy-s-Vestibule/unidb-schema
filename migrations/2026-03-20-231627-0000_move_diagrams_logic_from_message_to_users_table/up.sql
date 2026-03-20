-- Your SQL goes here

ALTER TABLE "scores" DROP COLUMN "intro_diagram";
ALTER TABLE "scores" DROP COLUMN "current_diagram";

ALTER TABLE "vestibule_users" ADD COLUMN "current_diagram" BYTEA;
ALTER TABLE "vestibule_users" ADD COLUMN "current_diagram_last_updated" TIMESTAMPTZ;
ALTER TABLE "vestibule_users" ADD COLUMN "intro_diagram" BYTEA;
