-- Your SQL goes here
ALTER TABLE "threads" DROP COLUMN "channel_id";
ALTER TABLE "threads" ADD COLUMN "parent_channel_id" TEXT NOT NULL;
