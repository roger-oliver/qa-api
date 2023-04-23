-- Add up migration script here
CREATE TABLE IF NOT EXISTS accounts
(
	id serial NOT NULL,
	email varchar(255) NOT NULL,
	password varchar(255) NOT NULL,
	created_on timestamp without time zone NULL   DEFAULT (now() at time zone 'utc')
);

ALTER TABLE accounts ADD CONSTRAINT pk__accounts
	PRIMARY KEY (id);

ALTER TABLE accounts 
  ADD CONSTRAINT uq__accounts__email UNIQUE (email);
