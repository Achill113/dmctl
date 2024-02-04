CREATE TABLE encounters (
  id                          INTEGER NOT NULL PRIMARY KEY,
  environment                 TEXT NOT NULL,
  situation                   TEXT
);

CREATE TABLE enemy_formulas (
  id                          INTEGER NOT NULL PRIMARY KEY,
  encounter_id                INTEGER NOT NULL,
  enemy_name                  TEXT NOT NULL,
  die_type                    TEXT,
  count                       INTEGER NOT NULL,
  FOREIGN KEY (encounter_id)
    REFERENCES encounters (id)
      ON DELETE CASCADE
      ON UPDATE NO ACTION
);

CREATE TABLE level_range (
  id                          INTEGER NOT NULL PRIMARY KEY,
  min_level                   INTEGER NOT NULL,
  max_level                   INTEGER NOT NULL,
  encounter_id                INTEGER NOT NULL,
  FOREIGN KEY (encounter_id)
    REFERENCES encounters (id)
      ON DELETE CASCADE
      ON UPDATE NO ACTION
);

CREATE TABLE roll_range (
  id                          INTEGER NOT NULL PRIMARY KEY,
  min_roll                   INTEGER NOT NULL,
  max_roll                   INTEGER NOT NULL,
  encounter_id                INTEGER NOT NULL,
  FOREIGN KEY (encounter_id)
    REFERENCES encounters (id)
      ON DELETE CASCADE
      ON UPDATE NO ACTION
);
