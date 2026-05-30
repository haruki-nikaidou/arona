-- Revert the initial schema migration. Dropping the schemas with CASCADE
-- removes every table, type, sequence, and index they contain.

DROP SCHEMA IF EXISTS vault CASCADE;
DROP SCHEMA IF EXISTS memory CASCADE;
DROP SCHEMA IF EXISTS auth CASCADE;
