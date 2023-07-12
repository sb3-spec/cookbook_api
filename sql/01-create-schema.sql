-- Chef
CREATE TABLE chef (
    -- ctime timestamp with time zone DEFAULT now(),
    firebase_id text primary key,
    username varchar(255),
    custom_tags text[] DEFAULT array[]::text[]
);

-- Recipe
CREATE TABLE recipe (
    id bigserial primary key,
    cid text NOT NULL,
    ctime timestamp with time zone DEFAULT now(),
    mid text, -- modifier user id
    mtime timestamp with time zone,
    title text NOT NULL,
    header text DEFAULT '',
    ingredients text[] DEFAULT array[]::text[],
    steps text[] DEFAULT array[]::text[],
    tags text[] DEFAULT array[]::text[],
    image_url text DEFAULT '',
    cook_time text DEFAULT '',
    prep_time text DEFAULT '',
    total_time text DEFAULT '',

    CONSTRAINT fk_chef
        FOREIGN KEY(cid)
        REFERENCES chef(firebase_id)
);

ALTER SEQUENCE recipe_id_seq RESTART WITH 4;
-- ALTER SEQUENCE chef_id_seq RESTART WITH 2;