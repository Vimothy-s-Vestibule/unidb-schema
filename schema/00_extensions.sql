-- ============================================================================
-- UniDB Schema
-- Run against a blank PostgreSQL database to initialize all tables
-- Tables are ordered to satisfy foreign key dependencies
-- ============================================================================

-- Load vector extension
CREATE EXTENSION IF NOT EXISTS "vector";

-- Default to public schema
SET search_path TO public;
