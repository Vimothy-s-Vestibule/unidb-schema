-- Your SQL goes here
ALTER TABLE "channels" ADD COLUMN "parent_channel_id" TEXT;

ALTER TABLE "messages" DROP COLUMN "thread_id";
ALTER TABLE "messages" DROP COLUMN "channel_id";
ALTER TABLE "messages" ADD COLUMN "channel_id" TEXT NOT NULL;



DROP TABLE IF EXISTS "threads";


ALTER TABLE "vestibule_users" ADD COLUMN "score_last_updated" TIMESTAMPTZ;

