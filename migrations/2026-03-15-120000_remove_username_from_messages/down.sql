DROP INDEX "messages_user_id_idx";
ALTER TABLE "messages" ADD COLUMN "username" TEXT NOT NULL DEFAULT '';
