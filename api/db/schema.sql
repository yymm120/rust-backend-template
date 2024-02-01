CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS backpacks
(
    id uuid DEFAULT uuid_generate_v1() NOT NULL CONSTRAINT backpack_pkey PRIMARY KEY,
    title text NOT NULL,
    vendor text NOT NULL,
    brand text NOT NULL,
    -- keywords text NOT NULL,
    production_year smallint NOT NULL,
    attribute text NOT NULL,
    created_at timestamp with time zone default CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);