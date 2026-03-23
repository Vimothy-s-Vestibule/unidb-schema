-- Your SQL goes here
ALTER TABLE "messages" ADD COLUMN "in_reply_to" TEXT;
ALTER TABLE "messages" ADD COLUMN "last_edited" TIMESTAMPTZ;
