-- This file should undo anything in `up.sql`
ALTER TABLE "threads" DROP CONSTRAINT parent_channel_id_fk;