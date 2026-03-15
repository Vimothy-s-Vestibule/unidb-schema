-- This file should undo anything in `up.sql`
ALTER TABLE "vestibule_users"
DROP CONSTRAINT "vestibule_users_intro_message_id_fkey";

ALTER TABLE "vestibule_users"
ADD CONSTRAINT "vestibule_users_intro_message_id_fkey"
FOREIGN KEY ("intro_message_id") REFERENCES "messages"("message_id");
