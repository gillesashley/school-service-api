-- Drop tables if they exist
DROP TABLE IF EXISTS courses;
DROP TABLE IF EXISTS teachers;

-- Create teachers table
CREATE TABLE teachers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    picture_url VARCHAR(255) NOT NULL,
    profile TEXT
);

-- Create courses table
CREATE TABLE courses (
    id SERIAL PRIMARY KEY,
    teacher_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    time TIMESTAMP,
    CONSTRAINT fk_teacher
        FOREIGN KEY(teacher_id)
        REFERENCES teachers(id)
        ON DELETE CASCADE
);

-- Insert sample data
INSERT INTO teachers (name, picture_url, profile)
VALUES ('John Doe', 'http://example.com/john.jpg', 'Experienced computer science professor.');

INSERT INTO teachers (name, picture_url, profile)
VALUES ('Jane Smith', 'http://example.com/jane.jpg', 'Specialist in machine learning and AI.');

INSERT INTO courses (teacher_id, name, time)
VALUES (1, 'Introduction to Rust', '2023-09-01 10:00:00');

INSERT INTO courses (teacher_id, name, time)
VALUES (1, 'Advanced Rust Programming', '2023-09-01 14:00:00');

INSERT INTO courses (teacher_id, name, time)
VALUES (2, 'Machine Learning with Rust', '2023-09-02 11:00:00'); 