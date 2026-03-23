-- This file should undo anything in `up.sql`

ALTER TABLE "messages" DROP COLUMN "channel_id";
DROP TABLE "channels";