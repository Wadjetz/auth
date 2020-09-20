-- Your SQL goes 

CREATE TABLE IF NOT EXISTS applications (
    id UUID PRIMARY KEY NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    website_url TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS authorization_attempts (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL,
    code TEXT NOT NULL,
    client_id TEXT NOT NULL,
    response_type TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    scope TEXT,
    state TEXT,
    created_at TIMESTAMP NOT NULL
);
