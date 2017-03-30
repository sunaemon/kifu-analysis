use super::schema::{users, kifu, gamers, users_kifu};
use std::time::SystemTime;

#[derive(Identifiable, Queryable, Associations, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[has_many(users_kifu)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub active: bool,
    pub balance: i32,
}

#[derive(Identifiable, Queryable, Associations, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gamer {
    pub id: i32,
    pub name: String,
    pub service: String,
}

#[derive(Queryable, Associations, Debug, Clone, PartialEq, Eq, Hash)]
#[table_name="users_kifu"]
pub struct UserKifu {
    pub user_id: i32,
    pub kifu_id: i32,
}

#[derive(Identifiable, Queryable, Associations, Debug, Clone, PartialEq, Eq)]
#[table_name="kifu"]
#[has_many(users_kifu)]
pub struct Kifu {
    pub id: i32,
    pub data: String,
    pub timestamp: Option<SystemTime>,
    pub black_id: Option<i32>,
    pub white_id: Option<i32>,
    pub original_uid: Option<String>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub hash: &'a [u8],
    pub salt: &'a [u8],
    pub active: bool,
}

#[derive(Insertable)]
#[table_name="gamers"]
pub struct NewGamer<'a> {
    pub name: &'a str,
    pub service: &'a str,
}

#[derive(Insertable)]
#[table_name="users_kifu"]
pub struct NewUserKifu {
    pub user_id: i32,
    pub kifu_id: i32,
}

#[derive(Insertable)]
#[table_name="kifu"]
pub struct NewKifu<'a> {
    pub data: &'a str,
    pub timestamp: Option<SystemTime>,
    pub black_id: Option<i32>,
    pub white_id: Option<i32>,
    pub original_uid: Option<&'a str>,
}
