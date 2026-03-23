-- Your SQL goes here



ALTER TABLE "user_platform_association" 
DROP CONSTRAINT IF EXISTS user_platform_association_platform_id_fkey;

ALTER TABLE "social_platforms" DROP COLUMN "id";
ALTER TABLE "social_platforms" ADD COLUMN "id" UUID NOT NULL PRIMARY KEY;

ALTER TABLE "user_platform_association" DROP COLUMN "platform_id";
ALTER TABLE "user_platform_association" ADD COLUMN "platform_id" UUID NOT NULL PRIMARY KEY;




