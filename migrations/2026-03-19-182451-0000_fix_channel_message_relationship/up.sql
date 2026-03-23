-- Your SQL goes here
ALTER TABLE "messages" ADD CONSTRAINT messages_channel_id_fk FOREIGN KEY ("channel_id") REFERENCES "channels"("channel_id");
