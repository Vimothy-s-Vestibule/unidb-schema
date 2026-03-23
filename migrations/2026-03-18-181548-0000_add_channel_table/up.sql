-- Your SQL goes here

CREATE TABLE "channels" (
    "channel_id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "channel_type" TEXT NOT NULL
);

ALTER TABLE "messages" ADD COLUMN "channel_id" TEXT NOT NULL REFERENCES "channels"("channel_id") ON DELETE CASCADE;
