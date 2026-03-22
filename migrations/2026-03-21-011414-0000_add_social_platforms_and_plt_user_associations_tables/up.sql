-- Your SQL goes here

CREATE TABLE "social_platforms"(
	"id" TEXT NOT NULL PRIMARY KEY,
	"platform_name" TEXT NOT NULL,
	"url" TEXT NOT NULL
);

CREATE TABLE "user_platform_association"(
	"vestibule_user_id" TEXT NOT NULL,
	"platform_id" TEXT NOT NULL,
	"platform_username" TEXT NOT NULL,
	"platform_display_name" TEXT NOT NULL,
	"profile_url" TEXT,
	"platform_association_mention_message_id" TEXT NOT NULL,
	"reasoning" TEXT NOT NULL,
	PRIMARY KEY("vestibule_user_id", "platform_id"),
	FOREIGN KEY ("vestibule_user_id") REFERENCES "vestibule_users"("discord_user_id"),
	FOREIGN KEY ("platform_id") REFERENCES "social_platforms"("id"),
	FOREIGN KEY ("platform_association_mention_message_id") REFERENCES "messages"("message_id")
);
