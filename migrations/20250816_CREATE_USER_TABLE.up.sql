CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "user" (
    id uuid not null primary key default (uuid_generate_v4()),
    username varchar(50) not null,
    password varchar not null,
    email varchar(150) not null unique,
    birth_date timestamp with time zone not null 
);