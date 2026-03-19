-- This file should undo anything in `up.sql`
ALTER TABLE "messages" DROP CONSTRAINT messages_channel_id_fk;
ALTER TABLE "messages" DROP CONSTRAINT messages_threads_id_fk;
