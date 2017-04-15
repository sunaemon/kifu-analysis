use super::schema::{users, gamers, kifu, analysis, kifu_position, users_kifu};
use chrono::prelude::*;

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

#[derive(Identifiable, Queryable, Associations, Debug, Clone, PartialEq, Eq)]
#[table_name="kifu"]
#[has_many(users_kifu)]
pub struct Kifu {
    pub id: i32,
    pub data: String,
    pub timestamp: Option<NaiveDateTime>,
    pub black_id: Option<i32>,
    pub white_id: Option<i32>,
    pub winner_id: Option<i32>,
    pub original_uid: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Debug, Clone, PartialEq, Eq)]
#[table_name="analysis"]
pub struct Analysis {
    pub id: i32,
    pub position: String,
    pub engine: String,
    pub option: String,
    pub timestamp: NaiveDateTime,
    pub infos: String,
}

#[derive(Queryable, Associations, Debug, Clone, PartialEq, Eq, Hash)]
#[table_name="kifu_position"]
#[belongs_to(Kifu)]
pub struct KifuPosition {
    pub id: i32,
    pub kifu_id: i32,
    pub n: i32,
    pub position: String,
}

#[derive(Queryable, Associations, Debug, Clone, PartialEq, Eq, Hash)]
#[table_name="users_kifu"]
#[belongs_to(User)]
#[belongs_to(Kifu)]
pub struct UserKifu {
    pub id: i32,
    pub user_id: i32,
    pub kifu_id: i32,
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
#[table_name="kifu"]
pub struct NewKifu<'a> {
    pub data: &'a str,
    pub timestamp: Option<NaiveDateTime>,
    pub black_id: Option<i32>,
    pub white_id: Option<i32>,
    pub winner_id: Option<i32>,
    pub original_uid: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="analysis"]
pub struct NewAnalysis<'a> {
    pub position: &'a str,
    pub engine: &'a str,
    pub option: &'a str,
    pub timestamp: NaiveDateTime,
    pub infos: &'a str,
}

#[derive(Insertable)]
#[table_name="kifu_position"]
pub struct NewKifuPosition<'a> {
    pub kifu_id: i32,
    pub n: i32,
    pub position: &'a str,
}

#[derive(Insertable)]
#[table_name="users_kifu"]
pub struct NewUserKifu {
    pub user_id: i32,
    pub kifu_id: i32,
}
