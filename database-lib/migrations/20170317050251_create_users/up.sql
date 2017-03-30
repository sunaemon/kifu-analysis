CREATE TABLE users (
  id serial primary key,
  email varchar not null unique,
  hash bytea not null,
  salt bytea not null,
  active boolean not null default 'f',
  balance integer not null default 0
);

CREATE TABLE gamers (
  id serial primary key,
  name varchar not null,
  service varchar not null,
  unique (name, service)
);

CREATE TABLE kifu (
  id serial primary key,
  data varchar not null,
  timestamp timestamp,
  black_id integer references gamers(id),
  white_id integer references gamers(id),
  original_uid varchar unique
);

CREATE TABLE users_kifu (
  id serial primary key, --Diesel only supports tables with primary keys.
  user_id integer references users(id) not null,
  kifu_id integer references kifu(id) not null
);

