-- This file should undo anything in `up.sql`

ALTER TABLE "threads" DROP COLUMN "parent_channel_id";
ALTER TABLE "threads" ADD COLUMN "channel_id" TEXT NOT NULL;
