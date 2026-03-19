-- This file should undo anything in `up.sql`
ALTER TABLE "channels" DROP COLUMN "parent_channel_id";

ALTER TABLE "messages" DROP COLUMN "channel_id";
ALTER TABLE "messages" ADD COLUMN "thread_id" TEXT;
ALTER TABLE "messages" ADD COLUMN "channel_id" TEXT;



CREATE TABLE "threads"(
	"thread_id" TEXT NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"thread_type" TEXT NOT NULL,
	"parent_channel_id" TEXT NOT NULL,
	FOREIGN KEY ("parent_channel_id") REFERENCES "channels"("channel_id")
);



ALTER TABLE "vestibule_users" DROP COLUMN "score_last_updated";

