CREATE TABLE
    gardyns (
        id SERIAL,
        name VARCHAR(255) NOT NULL,
        PRIMARY KEY (id)
    );

CREATE TABLE
    gardyn_slots (
        id SERIAL,
        gardyn_id INTEGER NOT NULL REFERENCES gardyns (id),
        plant_id INTEGER REFERENCES plants (id),
        x INTEGER NOT NULL,
        y INTEGER NOT NULL,
        PRIMARY KEY (id)
    );
