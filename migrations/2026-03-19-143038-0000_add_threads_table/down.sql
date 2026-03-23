-- This file should undo anything in `up.sql`

ALTER TABLE "messages" DROP COLUMN "thread_id";
ALTER TABLE "messages" DROP COLUMN "channel_id";
ALTER TABLE "messages" ADD COLUMN "channel_id" TEXT NOT NULL;

DROP TABLE IF EXISTS "threads";
