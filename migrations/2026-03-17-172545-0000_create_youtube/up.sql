-- Your SQL goes here

CREATE TABLE "youtube_comment_threads"(
	"youtube_comment_thread_id" TEXT NOT NULL PRIMARY KEY,
	"etag" TEXT NOT NULL,
	"date_added" TIMESTAMPTZ NOT NULL,
	"date_last_updated" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "youtube_channels"(
	"youtube_channel_id" TEXT NOT NULL PRIMARY KEY,
	"username" TEXT NOT NULL,
	"profile_image_url" TEXT NOT NULL,
	"channel_title" TEXT,
	"date_added" TIMESTAMPTZ NOT NULL,
	"date_last_updated" TIMESTAMPTZ
);

CREATE TABLE "youtube_videos"(
	"youtube_video_id" TEXT NOT NULL PRIMARY KEY,
	"channel_id" TEXT,
	"title" TEXT,
	"etag" TEXT,
	"view_count" INT8,
	"like_count" INT8,
	"comment_count" INT8,
	"date_published" TIMESTAMPTZ,
	"date_added" TIMESTAMPTZ NOT NULL,
	"date_last_updated" TIMESTAMPTZ,
	"last_checked" TIMESTAMPTZ,
	FOREIGN KEY ("channel_id") REFERENCES "youtube_channels"("youtube_channel_id")
);

CREATE TABLE "youtube_comments"(
	"youtube_comment_id" TEXT NOT NULL PRIMARY KEY,
	"parent_id" TEXT,
	"thread_id" TEXT NOT NULL,
	"etag" TEXT NOT NULL,
	"text" TEXT NOT NULL,
	"like_count" INT4 NOT NULL,
	"author_id" TEXT NOT NULL,
	"video_id" TEXT NOT NULL,
	"date_published" TIMESTAMPTZ NOT NULL,
	"date_added" TIMESTAMPTZ NOT NULL,
	"date_last_updated" TIMESTAMPTZ,
	"last_checked" TIMESTAMPTZ NOT NULL,
	"deleted" BOOL NOT NULL,
	FOREIGN KEY ("thread_id") REFERENCES "youtube_comment_threads"("youtube_comment_thread_id"),
	FOREIGN KEY ("author_id") REFERENCES "youtube_channels"("youtube_channel_id"),
	FOREIGN KEY ("video_id") REFERENCES "youtube_videos"("youtube_video_id")
);
