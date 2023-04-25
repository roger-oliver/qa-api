-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    account_id int4,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
); 

CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    account_id int4,
    question_id integer REFERENCES questions
);