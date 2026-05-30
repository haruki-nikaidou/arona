-- Initial schema migration: create the `auth`, `memory`, and `vault` schemas
-- and all tables/types currently defined in the `auth`, `memory_repository`,
-- and `vault` modules.

CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS memory;
CREATE SCHEMA IF NOT EXISTS vault;

-- =============================================================================
-- Schema: auth
-- =============================================================================

CREATE TYPE auth.account_status AS ENUM ('Owner', 'Member');

CREATE TABLE auth.account (
    id            uuid PRIMARY KEY,
    username      text NOT NULL UNIQUE,
    password      text NOT NULL,
    registered_at timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE TABLE auth.invitation (
    token            uuid PRIMARY KEY,
    created_at       timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    expire_at        timestamp NOT NULL,
    max_accept_count bigint,
    role             auth.account_status NOT NULL,
    send_by          uuid NOT NULL REFERENCES auth.account (id) ON DELETE CASCADE
);

CREATE TABLE auth.session (
    serial          bigserial PRIMARY KEY,
    user_id         uuid NOT NULL REFERENCES auth.account (id) ON DELETE CASCADE,
    session_id      text NOT NULL UNIQUE,
    last_refreshed  timestamp NOT NULL,
    expires         timestamp NOT NULL
);

CREATE INDEX session_user_id_idx ON auth.session (user_id);
CREATE INDEX session_expires_idx ON auth.session (expires);

-- =============================================================================
-- Schema: memory
-- =============================================================================

-- ----- Object storage --------------------------------------------------------

CREATE TABLE memory.object_storage_bucket (
    id             serial PRIMARY KEY,
    readable_name  text NOT NULL,
    bucket_name    text NOT NULL,
    region         text NOT NULL,
    api_endpoint   text NOT NULL,
    credential     bigint NOT NULL
);

CREATE TABLE memory.object_storage (
    hash              bytea PRIMARY KEY,
    file_name         text NOT NULL,
    file_size         bigint NOT NULL,
    description       text NOT NULL DEFAULT '',
    file_type         text NOT NULL,
    created_at        bigint NOT NULL,
    stored_in_bucket  integer NOT NULL REFERENCES memory.object_storage_bucket (id),
    CONSTRAINT object_storage_hash_len CHECK (octet_length(hash) = 32)
);

CREATE INDEX object_storage_bucket_idx ON memory.object_storage (stored_in_bucket);

-- ----- Conversations ---------------------------------------------------------

CREATE TYPE memory.message_role AS ENUM ('System', 'User', 'Assistant', 'Tool');

CREATE TYPE memory.content_modality AS ENUM ('Text', 'Image', 'Audio', 'File', 'Video');

CREATE TABLE memory.conversation (
    id              bigserial PRIMARY KEY,
    opening_summary text NOT NULL,
    closing_summary text,
    created_at      bigint NOT NULL,
    updated_at      bigint NOT NULL
);

CREATE TABLE memory.conversation_message (
    id                bigserial PRIMARY KEY,
    conversation_id   bigint NOT NULL REFERENCES memory.conversation (id) ON DELETE CASCADE,
    before            bigint REFERENCES memory.conversation_message (id),
    role              memory.message_role NOT NULL,
    created_at        timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    is_current_branch boolean NOT NULL DEFAULT TRUE
);

CREATE INDEX conversation_message_conversation_idx
    ON memory.conversation_message (conversation_id);
CREATE INDEX conversation_message_before_idx
    ON memory.conversation_message (before);

CREATE TABLE memory.conversation_content (
    id            bigserial PRIMARY KEY,
    message_id    bigint NOT NULL REFERENCES memory.conversation_message (id) ON DELETE CASCADE,
    position      integer NOT NULL,
    modality      memory.content_modality NOT NULL,
    text          text,
    object_hash   bytea,
    image_detail  text,
    audio_format  text,
    CONSTRAINT conversation_content_object_hash_len
        CHECK (object_hash IS NULL OR octet_length(object_hash) = 32),
    CONSTRAINT conversation_content_message_position_unique
        UNIQUE (message_id, position)
);

CREATE INDEX conversation_content_message_idx
    ON memory.conversation_content (message_id);

-- ----- Calendar --------------------------------------------------------------

CREATE TYPE memory.calender_event_repeat AS ENUM (
    'NoRepeat', 'EveryDay', 'EveryMonth', 'EveryWeekday'
);

CREATE TYPE memory.daily_event_repeat AS ENUM (
    'NoRepeat', 'EveryMonth', 'EveryYear', 'EveryWeekday'
);

CREATE TYPE memory.calender_task_status AS ENUM ('Pending', 'Doing', 'Finished');

CREATE TABLE memory.calender (
    id          uuid PRIMARY KEY,
    name        text NOT NULL,
    description text NOT NULL DEFAULT '',
    created_at  timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    scopes      text[] NOT NULL DEFAULT '{}'
);

CREATE TABLE memory.calender_event (
    id            bigserial PRIMARY KEY,
    calendar_id   uuid NOT NULL REFERENCES memory.calender (id) ON DELETE CASCADE,
    title         text NOT NULL,
    description   text NOT NULL DEFAULT '',
    time          timestamptz NOT NULL,
    repeat        memory.calender_event_repeat NOT NULL DEFAULT 'NoRepeat',
    repeat_until  date,
    created_at    timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at    timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE INDEX calender_event_calendar_idx ON memory.calender_event (calendar_id);
CREATE INDEX calender_event_time_idx ON memory.calender_event (time);

CREATE TABLE memory.calender_daily_event (
    id            bigserial PRIMARY KEY,
    calendar_id   uuid NOT NULL REFERENCES memory.calender (id) ON DELETE CASCADE,
    title         text NOT NULL,
    description   text NOT NULL DEFAULT '',
    date          date NOT NULL,
    repeat        memory.daily_event_repeat NOT NULL DEFAULT 'NoRepeat',
    repeat_until  date,
    created       timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated       timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE INDEX calender_daily_event_calendar_idx
    ON memory.calender_daily_event (calendar_id);
CREATE INDEX calender_daily_event_date_idx
    ON memory.calender_daily_event (date);

CREATE TABLE memory.calender_task (
    id                bigserial PRIMARY KEY,
    calendar_id       uuid NOT NULL REFERENCES memory.calender (id) ON DELETE CASCADE,
    title             text NOT NULL,
    description       text NOT NULL DEFAULT '',
    start_at          timestamp NOT NULL,
    deadline          timestamp NOT NULL,
    status            memory.calender_task_status NOT NULL DEFAULT 'Pending',
    status_update_at  timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE INDEX calender_task_calendar_idx ON memory.calender_task (calendar_id);
CREATE INDEX calender_task_deadline_idx ON memory.calender_task (deadline);

CREATE TABLE memory.calender_task_dependency (
    id               bigserial PRIMARY KEY,
    blocking_task_id bigint NOT NULL REFERENCES memory.calender_task (id) ON DELETE CASCADE,
    blocked_task_id  bigint NOT NULL REFERENCES memory.calender_task (id) ON DELETE CASCADE,
    CONSTRAINT calender_task_dependency_no_self_loop
        CHECK (blocking_task_id <> blocked_task_id),
    CONSTRAINT calender_task_dependency_unique
        UNIQUE (blocking_task_id, blocked_task_id)
);

-- ----- Contacts --------------------------------------------------------------

CREATE TYPE memory.relationship AS ENUM (
    'Stranger', 'Master', 'Acquaintance', 'Dude', 'Ignored'
);

CREATE TYPE memory.story_type AS ENUM (
    'RelationshipUpgrade',
    'RelationshipDowngrade',
    'FirstMeeting',
    'ImpressionChanged',
    'Other'
);

CREATE TABLE memory.contact_identity (
    id                      uuid PRIMARY KEY,
    identify_name           text NOT NULL,
    description             text NOT NULL DEFAULT '',
    relationship            memory.relationship NOT NULL DEFAULT 'Stranger',
    first_meet_at           timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    relationship_updated_at timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE TABLE memory.contact (
    id           bigserial PRIMARY KEY,
    display_name text NOT NULL,
    user_id      text NOT NULL,
    platform     text NOT NULL,
    identity     uuid NOT NULL REFERENCES memory.contact_identity (id) ON DELETE CASCADE,
    created_at   timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at   timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    CONSTRAINT contact_platform_user_unique UNIQUE (platform, user_id)
);

CREATE INDEX contact_identity_idx ON memory.contact (identity);

CREATE TABLE memory.contact_story (
    id                   bigserial PRIMARY KEY,
    identity             uuid NOT NULL REFERENCES memory.contact_identity (id) ON DELETE CASCADE,
    story_type           memory.story_type NOT NULL DEFAULT 'Other',
    story_name           text NOT NULL,
    story_summary        text NOT NULL DEFAULT '',
    story_text           text NOT NULL DEFAULT '',
    happened_at          timestamp NOT NULL,
    related_conversation bigint REFERENCES memory.conversation (id) ON DELETE SET NULL
);

CREATE INDEX contact_story_identity_idx ON memory.contact_story (identity);
CREATE INDEX contact_story_happened_at_idx ON memory.contact_story (happened_at);

-- ----- Diary -----------------------------------------------------------------

CREATE TABLE memory.diary (
    id         bigserial PRIMARY KEY,
    title      text NOT NULL,
    date       date NOT NULL UNIQUE,
    summary    text NOT NULL DEFAULT '',
    content    text NOT NULL DEFAULT '',
    created_at timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc')
);

-- =============================================================================
-- Schema: vault
-- =============================================================================

CREATE TYPE vault.secret_read_response AS ENUM (
    'Success',
    'WrongPassword',
    'SecretNotFound',
    'SignatureVerificationFailed'
);

CREATE TABLE vault.rolling_key (
    id             uuid PRIMARY KEY,
    encrypted_key  bytea NOT NULL,
    signature      bytea NOT NULL,
    created_at     bigint NOT NULL,
    before         uuid REFERENCES vault.rolling_key (id)
);

CREATE TABLE vault.secret (
    id              bigserial PRIMARY KEY,
    platform        text NOT NULL,
    name            text NOT NULL,
    allowed_scopes  text[] NOT NULL DEFAULT '{}',
    content         bytea NOT NULL,
    signature       bytea NOT NULL,
    key             uuid NOT NULL REFERENCES vault.rolling_key (id),
    created_at      bigint NOT NULL,
    updated_at      bigint NOT NULL,
    version         integer NOT NULL DEFAULT 1
);

CREATE INDEX secret_platform_idx ON vault.secret (platform);
CREATE INDEX secret_key_idx ON vault.secret (key);

CREATE TABLE vault.secret_read_log (
    id             bigserial PRIMARY KEY,
    target         bigint NOT NULL REFERENCES vault.secret (id) ON DELETE CASCADE,
    timestamp      timestamp NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    version        integer NOT NULL,
    response       vault.secret_read_response NOT NULL,
    wrong_password bytea,
    audience       text NOT NULL,
    scope          text NOT NULL
);

CREATE INDEX secret_read_log_target_idx ON vault.secret_read_log (target);
CREATE INDEX secret_read_log_timestamp_idx ON vault.secret_read_log (timestamp);

CREATE TABLE vault.ai_account (
    id                 serial PRIMARY KEY,
    name               text NOT NULL,
    websites           text[] NOT NULL DEFAULT '{}',
    username           text NOT NULL,
    password_encrypted bytea NOT NULL,
    password_hmac      bytea NOT NULL,
    is_master_version  boolean NOT NULL DEFAULT TRUE,
    created_at         bigint NOT NULL,
    is_removed         boolean NOT NULL DEFAULT FALSE,
    totp_url           text
);

CREATE INDEX ai_account_name_idx ON vault.ai_account (name);
