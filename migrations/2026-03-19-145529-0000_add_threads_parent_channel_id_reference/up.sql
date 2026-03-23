-- Your SQL goes here
ALTER TABLE "threads" ADD CONSTRAINT parent_channel_id_fk FOREIGN KEY ("parent_channel_id") REFERENCES "channels"("channel_id")
