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
  user_id integer references users(id) not null,
  data varchar not null,
  timestamp timestamp not null,
  black_id integer references gamers(id),
  white_id integer references gamers(id)
)
