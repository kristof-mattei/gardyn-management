CREATE TABLE
    gardyn (
        id SERIAL,
        name VARCHAR(255) NOT NULL,
        PRIMARY KEY (id)
    );

CREATE TABLE
    gardyn_slot (
        id SERIAL,
        gardyn_id INTEGER NOT NULL REFERENCES gardyn (id),
        plant_id INTEGER REFERENCES plant (id),
        x INTEGER NOT NULL,
        y INTEGER NOT NULL,
        PRIMARY KEY (id)
    );
