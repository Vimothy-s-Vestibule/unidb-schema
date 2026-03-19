-- Your SQL goes here

ALTER TABLE "messages" DROP COLUMN "channel_id";
ALTER TABLE "messages" ADD COLUMN "thread_id" TEXT;
ALTER TABLE "messages" ADD COLUMN "channel_id" TEXT;

CREATE TABLE "threads"(
	"thread_id" TEXT NOT NULL PRIMARY KEY,
	"channel_id" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"thread_type" TEXT NOT NULL,
	FOREIGN KEY ("channel_id") REFERENCES "channels"("channel_id")
);

