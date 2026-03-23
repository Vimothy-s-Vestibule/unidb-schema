-- This file should undo anything in `up.sql`

ALTER TABLE "scores" ADD COLUMN "intro_diagram" BYTEA;
ALTER TABLE "scores" ADD COLUMN "current_diagram" BYTEA;

ALTER TABLE "vestibule_users" DROP COLUMN "current_diagram";
ALTER TABLE "vestibule_users" DROP COLUMN "current_diagram_last_updated";
ALTER TABLE "vestibule_users" DROP COLUMN "intro_diagram";
