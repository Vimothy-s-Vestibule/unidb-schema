-- This file should undo anything in `up.sql`
ALTER TABLE "messages" DROP COLUMN "in_reply_to";
ALTER TABLE "messages" DROP COLUMN "last_edited";
