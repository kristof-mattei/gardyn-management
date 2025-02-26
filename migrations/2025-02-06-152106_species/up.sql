CREATE TABLE
    category (
        id SERIAL,
        name VARCHAR(255) NOT NULL,
        PRIMARY KEY (id)
    );

CREATE TABLE
    species (
        id SERIAL,
        name VARCHAR(255) NOT NULL,
        category_id INTEGER NOT NULL REFERENCES category (id),
        PRIMARY KEY (id)
    );

