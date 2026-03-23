-- WARNING: This schema is for context only and is not meant to be run.
-- Table order and constraints may not be valid for execution.

CREATE TABLE public.__diesel_schema_migrations (
  version character varying NOT NULL,
  run_on timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version)
);
CREATE TABLE public.channels (
  channel_id text NOT NULL,
  name text NOT NULL,
  channel_type text NOT NULL,
  parent_channel_id text,
  CONSTRAINT channels_pkey PRIMARY KEY (channel_id)
);
CREATE TABLE public.messages (
  message_id text NOT NULL,
  user_id text NOT NULL,
  content text NOT NULL,
  sent_at timestamp with time zone NOT NULL,
  added_at timestamp with time zone NOT NULL,
  score_id text,
  in_reply_to text,
  last_edited timestamp with time zone,
  deleted_at timestamp with time zone,
  channel_id text NOT NULL,
  CONSTRAINT messages_pkey PRIMARY KEY (message_id),
  CONSTRAINT messages_score_id_fkey FOREIGN KEY (score_id) REFERENCES public.scores(score_id),
  CONSTRAINT messages_channel_id_fk FOREIGN KEY (channel_id) REFERENCES public.channels(channel_id)
);
CREATE TABLE public.scores (
  score_id text NOT NULL,
  honesty_humility double precision NOT NULL,
  emotionality double precision NOT NULL,
  extraversion double precision NOT NULL,
  agreeableness double precision NOT NULL,
  conscientiousness double precision NOT NULL,
  openness_to_experience double precision NOT NULL,
  agency double precision NOT NULL,
  communion double precision NOT NULL,
  self_direction double precision NOT NULL,
  stimulation double precision NOT NULL,
  hedonism double precision NOT NULL,
  achievement double precision NOT NULL,
  power double precision NOT NULL,
  security double precision NOT NULL,
  conformity double precision NOT NULL,
  tradition double precision NOT NULL,
  benevolence double precision NOT NULL,
  universalism double precision NOT NULL,
  activities ARRAY NOT NULL,
  domains ARRAY NOT NULL,
  embedding USER-DEFINED,
  CONSTRAINT scores_pkey PRIMARY KEY (score_id)
);
CREATE TABLE public.skills (
  id uuid NOT NULL DEFAULT gen_random_uuid(),
  name text NOT NULL UNIQUE,
  embedding USER-DEFINED,
  CONSTRAINT skills_pkey PRIMARY KEY (id)
);
CREATE TABLE public.social_platforms (
  platform_name text NOT NULL,
  url text NOT NULL,
  id uuid NOT NULL,
  CONSTRAINT social_platforms_pkey PRIMARY KEY (id)
);
CREATE TABLE public.user_platform_association (
  vestibule_user_id text NOT NULL,
  platform_username text NOT NULL,
  platform_display_name text NOT NULL,
  profile_url text,
  platform_association_mention_message_id text NOT NULL,
  reasoning text NOT NULL,
  platform_id uuid NOT NULL,
  CONSTRAINT user_platform_association_pkey PRIMARY KEY (platform_id),
  CONSTRAINT user_platform_association_vestibule_user_id_fkey FOREIGN KEY (vestibule_user_id) REFERENCES public.vestibule_users(discord_user_id),
  CONSTRAINT user_platform_association_platform_association_mention_mes_fkey FOREIGN KEY (platform_association_mention_message_id) REFERENCES public.messages(message_id)
);
CREATE TABLE public.user_skill_evidence (
  message_id text NOT NULL,
  weight real NOT NULL,
  reasoning text NOT NULL,
  user_skill_id uuid NOT NULL,
  CONSTRAINT user_skill_evidence_pkey PRIMARY KEY (user_skill_id, message_id),
  CONSTRAINT user_skill_evidence_message_id_fkey FOREIGN KEY (message_id) REFERENCES public.messages(message_id),
  CONSTRAINT user_skill_evidence_user_skill_id_fkey FOREIGN KEY (user_skill_id) REFERENCES public.user_skills(id)
);
CREATE TABLE public.user_skills (
  user_id text NOT NULL,
  skill_id uuid NOT NULL,
  level smallint NOT NULL CHECK (level >= 0 AND level <= 10),
  llm_context text,
  id uuid NOT NULL DEFAULT gen_random_uuid(),
  CONSTRAINT user_skills_pkey PRIMARY KEY (id),
  CONSTRAINT user_skills_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.vestibule_users(discord_user_id),
  CONSTRAINT user_skills_skill_id_fkey FOREIGN KEY (skill_id) REFERENCES public.skills(id)
);
CREATE TABLE public.vestibule_users (
  discord_user_id text NOT NULL,
  discord_username text NOT NULL,
  yt_username text,
  yt_display_name text,
  intro_message_id text,
  status text NOT NULL,
  score_id text,
  score_last_updated timestamp with time zone,
  discord_display_name text NOT NULL,
  current_diagram bytea,
  current_diagram_last_updated timestamp with time zone,
  intro_diagram bytea,
  CONSTRAINT vestibule_users_pkey PRIMARY KEY (discord_user_id),
  CONSTRAINT vestibule_users_intro_message_id_fkey FOREIGN KEY (intro_message_id) REFERENCES public.messages(message_id),
  CONSTRAINT vestibule_users_score_id_fkey FOREIGN KEY (score_id) REFERENCES public.scores(score_id)
);