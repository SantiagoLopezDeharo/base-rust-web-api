CREATE TABLE
    IF NOT EXISTS "USER" (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        username TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL
    );