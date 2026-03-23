-- This file should undo anything in `up.sql`
ALTER TABLE "messages" DROP CONSTRAINT messages_channel_id_fk;
