# ToDos

- [ ] Choose vector embedding dimension: field_name vector(12301)

- [ ] FIX: Associate discord pfp media asset with discord user object via FK so we can asscoiate them for display on the dashboard

- [ ] Centralize the job status or processing status of a message, account, basically anything into the job table or some other table in order to have to add metadata to everything like this:

```sql

  -- Processing pipeline metadata
  -- TODO make it so admins can manually override messages to be included/excluded from skills or personality processing
  triage_status text NOT NULL DEFAULT 'pending',    -- pending: Will be processed/processing: A worker is currently processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
 -- |
 -- |
 -- ⌄
  is_significant boolean NOT NULL, -- Whether an LLM should score and extract personality from it, TODO impl this field is also being set by an LLM
 -- |
 -- |
 -- ⌄
  skill_status text, -- NULL: insignificant for skills/pending: Will be processed/processing: A worker is currently processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
  personality_status text, -- NULL: insignificant for persinality extraction/pending: Will be processed/processing: A worker is currently processing this messaage and the status will change soon/complete: The message has been processed (terminal)/skipped: Message is insignificant (skipped, terminal)/failed: Will be retried when a cleanup job is run on the db
  processed_at timestamptz

```
