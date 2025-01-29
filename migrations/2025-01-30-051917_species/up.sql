CREATE TABLE
    "categories" (
        "id" SERIAL,
        "name" VARCHAR(255) NOT NULL,
        PRIMARY KEY ("id")
    );

CREATE TABLE
    "species" (
        "id" SERIAL,
        "name" VARCHAR(255) NOT NULL,
        "category_id" INTEGER NOT NULL REFERENCES "categories" ("id"),
        PRIMARY KEY ("id")
    );
