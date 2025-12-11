CREATE TABLE IF NOT EXISTS relationship (
    id INTEGER PRIMARY KEY AUTO_INCREMENT NOT NULL,
    relationship_type ENUM('DATING', 'ENGAGED', 'MARRIED') NOT NULL,
    start_date DATE NOT NULL
);

CREATE TABLE IF NOT EXISTS person_relationship_link (
    person_id INTEGER NOT NULL,
    FOREIGN KEY (person_id) REFERENCES person(id),
    relationship_id INTEGER NOT NULL,
    FOREIGN KEY (relationship_id) REFERENCES relationship(id),
    PRIMARY KEY (person_id, relationship_id)
);