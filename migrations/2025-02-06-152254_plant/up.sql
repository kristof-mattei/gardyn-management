CREATE TABLE plant (
  id SERIAL,
  name VARCHAR(255) NOT NULL,
  species_id INTEGER REFERENCES species (id),
  creation TIMESTAMP NOT NULL,
  creation_offset INTEGER NOT NULL,
  creation_time_zone TEXT NOT NULL,
  ending TIMESTAMP,
  ending_offset INTEGER NULL,
  ending_time_zone TEXT,
  PRIMARY KEY (id),
  CONSTRAINT creation_time_zone_valid CHECK (
    NOW () AT TIME ZONE creation_time_zone IS NOT NULL
  ),
  CONSTRAINT ending_valid CHECK (
    (
      ending IS NULL
      AND ending_offset IS NULL
      AND ending_time_zone IS NULL
    )
    OR (
      ending IS NOT NULL
      AND ending_offset IS NOT NULL
      AND ending_time_zone IS NOT NULL
    )
  ),
  CONSTRAINT ending_time_zone_valid CHECK (
    ending_time_zone IS NULL
    OR NOW () AT TIME ZONE ending_time_zone IS NOT NULL
  )
);
