-- Migration 20261104_schema_integrity
--
-- Brings a database initialised from an older schema/*.sql (e.g. a7a7c55) to the
-- integrity rules of the current schema/*.sql. The schema files are only applied
-- when the Postgres volume is first created, so existing databases need this.
--
--   psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f migrations/20261104_schema_integrity.sql
--
-- * Idempotent: running it again (or on a database created from the current
--   schema files) changes nothing.
-- * Runs in one transaction: it either applies completely or not at all.
-- * Existing rows that would violate a new rule are handled, not fatal:
--     - duplicate discord_emojis / topic rows are merged into one canonical row
--       (references are re-pointed), duplicate reactions and extra topic links
--       are removed; every step reports what it did with RAISE NOTICE;
--     - out-of-range scores/confidence/weight values are NOT changed: the CHECK
--       is added NOT VALID (enforced for new writes only) and a WARNING reports
--       the count. Fix the rows, then `ALTER TABLE .. VALIDATE CONSTRAINT ..`.
--
-- Changes:
--  1. CHECK 0..1 on scores.*, user_facts_and_activities.confidence,
--     fact_and_activity_evidence.weight
--  2. UNIQUE discord_emojis.discord_emoji_id
--  3. UNIQUE topic.name
--  4. UNIQUE topic_message_relation.message_id (one topic per message)
--  5. message_reactions unique key becomes NULLS NOT DISTINCT
--  6. DROP UNIQUE discord_accounts.username
--  7. social_platforms seed rows get their fixed (UUIDv5) ids

BEGIN;

-- Serialise concurrent runs and keep writers from slipping duplicates in
-- between the clean-up and the constraint.
DO $$ BEGIN PERFORM pg_advisory_xact_lock(hashtext('unidb_migration_20261104_schema_integrity')); END $$;
LOCK TABLE discord_emojis, message_reactions, topic, topic_message_relation,
           social_platforms, connected_accounts IN SHARE ROW EXCLUSIVE MODE;

-- Adds a CHECK constraint if missing; NOT VALID (with a WARNING) if existing rows violate it.
CREATE OR REPLACE FUNCTION pg_temp.add_check(tbl regclass, con name, expr text) RETURNS void
LANGUAGE plpgsql AS $$
DECLARE
  bad bigint;
BEGIN
  IF EXISTS (SELECT 1 FROM pg_constraint WHERE conrelid = tbl AND conname = con) THEN
    RETURN;
  END IF;
  EXECUTE format('SELECT count(*) FROM %s WHERE NOT (%s)', tbl, expr) INTO bad;
  IF bad > 0 THEN
    RAISE WARNING '%: % existing row(s) violate %; constraint added NOT VALID (new writes are checked). Fix them, then: ALTER TABLE % VALIDATE CONSTRAINT %;',
      tbl, bad, con, tbl, con;
    EXECUTE format('ALTER TABLE %s ADD CONSTRAINT %I CHECK (%s) NOT VALID', tbl, con, expr);
  ELSE
    EXECUTE format('ALTER TABLE %s ADD CONSTRAINT %I CHECK (%s)', tbl, con, expr);
    RAISE NOTICE '%: added %', tbl, con;
  END IF;
END $$;

-- 1. value ranges --------------------------------------------------------------
DO $$ BEGIN
  PERFORM pg_temp.add_check('scores', 'scores_values_in_unit_range',
  'honesty BETWEEN 0 AND 1 AND emotionality BETWEEN 0 AND 1 AND
   extraversion BETWEEN 0 AND 1 AND agreeableness BETWEEN 0 AND 1 AND
   conscientiousness BETWEEN 0 AND 1 AND openness_to_experience BETWEEN 0 AND 1 AND
   agency BETWEEN 0 AND 1 AND achievement BETWEEN 0 AND 1 AND
   influence BETWEEN 0 AND 1 AND sarcasm BETWEEN 0 AND 1 AND
   security BETWEEN 0 AND 1 AND self_reflection BETWEEN 0 AND 1 AND
   technical_competence BETWEEN 0 AND 1 AND busyness BETWEEN 0 AND 1');
  PERFORM pg_temp.add_check('user_facts_and_activities', 'user_facts_and_activities_confidence_range',
  'confidence BETWEEN 0 AND 1');
  PERFORM pg_temp.add_check('fact_and_activity_evidence', 'fact_and_activity_evidence_weight_range',
  'weight BETWEEN 0 AND 1');
END $$;

-- 2. + 5. discord_emojis.discord_emoji_id unique, reactions NULLS NOT DISTINCT --
DO $$
DECLARE
  n bigint;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_constraint
                 WHERE conrelid = 'discord_emojis'::regclass
                   AND conname = 'discord_emojis_discord_emoji_id_key') THEN
    -- canonical row per emoji: the one with a stored asset, then lowest id
    CREATE TEMP TABLE _emoji_map ON COMMIT DROP AS
      SELECT id AS dup_id, keep_id FROM (
        SELECT id, first_value(id) OVER (PARTITION BY discord_emoji_id
                                         ORDER BY (asset_id IS NULL), id) AS keep_id
        FROM discord_emojis) s
      WHERE id <> keep_id;
    SELECT count(*) INTO n FROM _emoji_map;
    IF n > 0 THEN
      RAISE NOTICE 'discord_emojis: merging % duplicate row(s) into their canonical emoji', n;
    END IF;
  ELSE
    CREATE TEMP TABLE _emoji_map (dup_id uuid, keep_id uuid) ON COMMIT DROP;
  END IF;

  IF EXISTS (SELECT 1 FROM pg_index i JOIN pg_class c ON c.oid = i.indexrelid
             WHERE c.relname = 'message_reactions_message_id_user_id_emoji_id_key'
               AND i.indnullsnotdistinct)
     AND NOT EXISTS (SELECT 1 FROM _emoji_map) THEN
    RETURN; -- already migrated
  END IF;

  -- reactions that become duplicates once re-pointed to the canonical emoji, or
  -- that already are duplicates with emoji_id NULL: keep the earliest
  WITH target AS (
    SELECT r.id, r.message_id, r.user_id, r.reacted_at,
           coalesce(m.keep_id, r.emoji_id) AS emoji
    FROM message_reactions r LEFT JOIN _emoji_map m ON m.dup_id = r.emoji_id
  ), ranked AS (
    SELECT id, row_number() OVER (PARTITION BY message_id, user_id, emoji
                                  ORDER BY reacted_at, id) AS rn
    FROM target
  )
  DELETE FROM message_reactions WHERE id IN (SELECT id FROM ranked WHERE rn > 1);
  GET DIAGNOSTICS n = ROW_COUNT;
  IF n > 0 THEN
    RAISE NOTICE 'message_reactions: removed % duplicate reaction(s)', n;
  END IF;

  UPDATE message_reactions r SET emoji_id = m.keep_id
  FROM _emoji_map m WHERE r.emoji_id = m.dup_id;
  DELETE FROM discord_emojis e USING _emoji_map m WHERE e.id = m.dup_id;

  IF NOT EXISTS (SELECT 1 FROM pg_constraint
                 WHERE conrelid = 'discord_emojis'::regclass
                   AND conname = 'discord_emojis_discord_emoji_id_key') THEN
    ALTER TABLE discord_emojis
      ADD CONSTRAINT discord_emojis_discord_emoji_id_key UNIQUE (discord_emoji_id);
    RAISE NOTICE 'discord_emojis: added discord_emojis_discord_emoji_id_key';
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_index i JOIN pg_class c ON c.oid = i.indexrelid
                 WHERE c.relname = 'message_reactions_message_id_user_id_emoji_id_key'
                   AND i.indnullsnotdistinct) THEN
    ALTER TABLE message_reactions
      DROP CONSTRAINT IF EXISTS message_reactions_message_id_user_id_emoji_id_key;
    ALTER TABLE message_reactions
      ADD CONSTRAINT message_reactions_message_id_user_id_emoji_id_key
      UNIQUE NULLS NOT DISTINCT (message_id, user_id, emoji_id);
    RAISE NOTICE 'message_reactions: unique key is now NULLS NOT DISTINCT';
  END IF;
END $$;

-- 3. + 4. topic.name unique, one topic per message ------------------------------
DO $$
DECLARE
  n bigint;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_constraint
                 WHERE conrelid = 'topic_message_relation'::regclass
                   AND conname = 'topic_message_relation_message_id_key') THEN
    -- keep the first-inserted link per message (what the dashboard showed:
    -- "first topic wins")
    DELETE FROM topic_message_relation WHERE ctid IN (
      SELECT ctid FROM (
        SELECT ctid, row_number() OVER (PARTITION BY message_id ORDER BY ctid) AS rn
        FROM topic_message_relation) s
      WHERE rn > 1);
    GET DIAGNOSTICS n = ROW_COUNT;
    IF n > 0 THEN
      RAISE NOTICE 'topic_message_relation: removed % extra topic link(s) (kept the first per message)', n;
    END IF;
    ALTER TABLE topic_message_relation
      ADD CONSTRAINT topic_message_relation_message_id_key UNIQUE (message_id);
    RAISE NOTICE 'topic_message_relation: added topic_message_relation_message_id_key';
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_constraint
                 WHERE conrelid = 'topic'::regclass AND conname = 'topic_name_key') THEN
    CREATE TEMP TABLE _topic_map ON COMMIT DROP AS
      SELECT id AS dup_id, keep_id FROM (
        SELECT id, first_value(id) OVER (PARTITION BY name
                                         ORDER BY (embedding IS NULL), id) AS keep_id
        FROM topic) s
      WHERE id <> keep_id;
    SELECT count(*) INTO n FROM _topic_map;
    IF n > 0 THEN
      RAISE NOTICE 'topic: merging % duplicate topic(s) by name', n;
    END IF;
    UPDATE topic_message_relation r SET topic_id = m.keep_id
    FROM _topic_map m WHERE r.topic_id = m.dup_id;
    DELETE FROM topic t USING _topic_map m WHERE t.id = m.dup_id;
    ALTER TABLE topic ADD CONSTRAINT topic_name_key UNIQUE (name);
    RAISE NOTICE 'topic: added topic_name_key';
  END IF;
END $$;

-- 6. usernames are not unique -------------------------------------------------
ALTER TABLE discord_accounts DROP CONSTRAINT IF EXISTS discord_accounts_username_key;

-- 7. deterministic social_platforms ids -----------------------------------------
DO $$
DECLARE
  p record;
  old_id uuid;
BEGIN
  FOR p IN SELECT * FROM (VALUES
      ('961d567c-313f-5efc-9394-b14370bf9296'::uuid, 'Strava',   'https://strava.com',   'oauth_required'::platform_access),
      ('3ae8decc-dc79-5109-806a-0097409a95f2'::uuid, 'Spotify',  'https://spotify.com',  'oauth_required'::platform_access),
      ('a27177ef-c06c-55b6-8ed3-18e975683b0d'::uuid, 'LinkedIn', 'https://linkedin.com', 'unavailable'::platform_access),
      ('05f4b960-1dfb-50c5-b9b2-53813526bf14'::uuid, 'YouTube',  'https://youtube.com',  'public'::platform_access),
      ('b04e255d-9798-5546-b8a1-610877262df9'::uuid, 'GitHub',   'https://github.com',   'public'::platform_access),
      ('a52d91c3-8c1f-50cf-b98a-ae034fc5d49a'::uuid, 'Substack', 'https://substack.com', 'public'::platform_access)
    ) AS v(id, platform_name, homepage, access_type)
  LOOP
    SELECT id INTO old_id FROM social_platforms WHERE platform_name = p.platform_name;
    IF old_id IS NULL THEN
      INSERT INTO social_platforms (id, platform_name, homepage, access_type)
      VALUES (p.id, p.platform_name, p.homepage, p.access_type)
      ON CONFLICT DO NOTHING;
    ELSIF old_id <> p.id THEN
      -- the FK from connected_accounts has no ON UPDATE CASCADE: copy the row to
      -- the new id (keeping any local edits), re-point references, drop the old one
      UPDATE social_platforms SET platform_name = platform_name || ' (migrating ' || old_id || ')'
      WHERE id = old_id;
      INSERT INTO social_platforms (id, platform_name, homepage, access_type, logo)
      SELECT p.id, p.platform_name, homepage, access_type, logo FROM social_platforms WHERE id = old_id;
      UPDATE connected_accounts SET platform_id = p.id WHERE platform_id = old_id;
      DELETE FROM social_platforms WHERE id = old_id;
      RAISE NOTICE 'social_platforms: % id % -> %', p.platform_name, old_id, p.id;
    END IF;
  END LOOP;
END $$;

COMMIT;
