ALTER TABLE "messages" DROP COLUMN "username";
CREATE INDEX "messages_user_id_idx" ON "messages" ("user_id");
