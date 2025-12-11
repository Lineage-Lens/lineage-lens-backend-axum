CREATE TABLE IF NOT EXISTS person (
    id INTEGER PRIMARY KEY AUTO_INCREMENT NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    birth_date DATE,
    gender ENUM('MALE', 'FEMALE') NOT NULL,
    father_id INTEGER,
    FOREIGN KEY (father_id) REFERENCES person(id),
    mother_id INTEGER,
    FOREIGN KEY (mother_id) REFERENCES person(id)
)