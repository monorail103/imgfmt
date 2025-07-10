DROP TABLE IF EXISTS r2_files;

CREATE TABLE r2_files (
    file_name TEXT PRIMARY KEY,
    valid_date TEXT NOT NULL,
    delete_password TEXT NOT NULL,
    is_valid BOOLEAN NOT NULL DEFAULT TRUE
);

