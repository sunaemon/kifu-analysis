use super::schema::{users, kifu};

#[derive(Identifiable, Queryable, Associations, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[has_many(kifu)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub active: bool,
    pub balance: i32,
}

#[derive(Identifiable, Queryable, Associations, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[table_name="kifu"]
#[belongs_to(User)]
pub struct Kifu {
    pub id: i32,
    pub user_id: i32,
    pub data: String,
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
#[table_name="kifu"]
pub struct NewKifu<'a> {
    pub user_id: i32,
    pub data: &'a str,
}
