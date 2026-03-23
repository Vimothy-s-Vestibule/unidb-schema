-- This file should undo anything in `up.sql`




ALTER TABLE "social_platforms" DROP COLUMN "id";
ALTER TABLE "social_platforms" ADD COLUMN "id" TEXT NOT NULL;

ALTER TABLE "user_platform_association" DROP COLUMN "platform_id";
ALTER TABLE "user_platform_association" ADD COLUMN "platform_id" TEXT NOT NULL;




