CREATE TABLE users (
  id serial primary key,
  email varchar not null unique,
  hash bytea not null,
  salt bytea not null,
  active boolean not null default 'f',
  balance integer not null default 0
);

CREATE TABLE kifu (
  id serial primary key,
  user_id integer references users(id),
  data varchar
)
