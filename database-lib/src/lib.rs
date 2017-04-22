#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate rand;
extern crate crypto;
extern crate dotenv;
extern crate chrono;
extern crate core_lib;
extern crate rustc_serialize;

pub mod schema;
pub mod models;

use std::convert::From;
use std::error::Error;
use std::env;
use std::fmt;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use rand::{Rng, OsRng};

use crypto::hmac::Hmac;
use crypto::sha2::Sha256;

use chrono::prelude::*;

use rustc_serialize::json;

use models::{User, NewUser, Kifu, NewKifu, Gamer, NewGamer, UserKifu, NewUserKifu, Analysis,
             NewAnalysis};
use schema::{users, kifu, gamers, users_kifu, analysis};

use core_lib::types::Position;
use core_lib::encoder;
use core_lib::usi_engine::UsiEngineInfo;

lazy_static! {
  static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set").to_string();
}

pub struct Database {
    pub conn: PgConnection,
}

#[derive(Debug)]
pub struct DatabaseError {
    message: String,
    cause: Option<Box<Error + Send>>,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.message)
    }
}

impl Error for DatabaseError {
    fn description(self: &DatabaseError) -> &str {
        &self.message
    }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(data: diesel::result::Error) -> DatabaseError {
        DatabaseError {
            message: data.description().to_string(),
            cause: Some(Box::new(data)),
        }
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(data: std::io::Error) -> DatabaseError {
        DatabaseError {
            message: data.description().to_string(),
            cause: Some(Box::new(data)),
        }
    }
}

impl Database {
    pub fn new() -> Database {
        // TODO: use connection pool
        let conn = PgConnection::establish(&DATABASE_URL)
            .expect(&format!("Error connecting to {}", *DATABASE_URL));
        Database { conn: conn }
    }

    pub fn create_user(&self, email: &str, password: &str) -> Result<User, DatabaseError> {
        let mut mac = Hmac::new(Sha256::new(), password.as_bytes());
        let mut rng = OsRng::new()?;
        let mut salt: [u8; 32] = [0; 32];
        let mut hash: [u8; 512] = [0; 512];
        rng.fill_bytes(&mut salt);
        crypto::pbkdf2::pbkdf2(&mut mac, &salt[..], 1000, &mut hash);

        let new_user = NewUser {
            email: email,
            hash: &hash[..],
            salt: &salt[..],
            active: true,
        };

        let u = diesel::insert(&new_user).into(users::table).get_result(&self.conn)?;
        Ok(u)
    }

    pub fn assume_user(&self, email: &str, password: &str) -> Result<(), DatabaseError> {
        let user = self.get_user(email)?;

        let mut hash: [u8; 512] = [0; 512];
        let mut mac = Hmac::new(Sha256::new(), password.as_bytes());
        crypto::pbkdf2::pbkdf2(&mut mac, &user.salt, 1000, &mut hash);

        if &user.hash[..] != &hash[..] {
            return Err(DatabaseError {
                message: "Wrong Password".to_string(),
                cause: None,
            });
        }

        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, DatabaseError> {
        let u = users::table.filter(users::email.eq(email)).first::<User>(&self.conn)?;

        Ok(u)
    }

    pub fn get_kifu(&self, id: i32) -> Result<Kifu, DatabaseError> {
        let k = kifu::table.find(id).first(&self.conn)?;

        Ok(k)
    }

    pub fn fav_kifu(&self, user: &User, kifu: &Kifu, fav: bool) -> Result<(), DatabaseError> {
        if fav && !self.get_fav_kifu(user, kifu)? {
            let new_user_kifu = NewUserKifu {
                user_id: user.id,
                kifu_id: kifu.id,
            };

            diesel::insert(&new_user_kifu).into(users_kifu::table)
                .get_result::<UserKifu>(&self.conn)?;
        } else {
            diesel::delete(users_kifu::table.filter(users_kifu::user_id.eq(user.id))
                    .filter(users_kifu::kifu_id.eq(kifu.id))).execute(&self.conn)?;
        }
        Ok(())
    }

    pub fn get_fav_kifu(&self, user: &User, kifu: &Kifu) -> Result<bool, DatabaseError> {
        let k = users_kifu::table.filter(users_kifu::user_id.eq(user.id))
            .filter(users_kifu::kifu_id.eq(kifu.id))
            .get_result::<UserKifu>(&self.conn)
            .optional()?;
        Ok(k.is_some())
    }

    pub fn list_kifu(&self, user: &User) -> Result<Vec<Kifu>, DatabaseError> {
        let us = kifu::table.inner_join(users_kifu::table)
            .filter(users_kifu::user_id.eq(user.id))
            .load::<(Kifu, UserKifu)>(&self.conn)?;
        let mut ks = Vec::new();
        for (k, _) in us {
            ks.push(k);
        }
        Ok(ks)
    }

    pub fn create_gamer(&self, name: &str, service: &str) -> Result<Gamer, DatabaseError> {
        let new_gamer = NewGamer {
            name: name,
            service: service,
        };

        let gamer = diesel::insert(&new_gamer).into(gamers::table)
            .get_result(&self.conn)?;
        Ok(gamer)
    }

    pub fn find_gamer(&self, id: i32) -> Result<Gamer, DatabaseError> {
        let g = gamers::table.find(id).first(&self.conn)?;

        Ok(g)
    }

    pub fn create_or_find_gamer(&self, name: &str, service: &str) -> Result<Gamer, DatabaseError> {
        let gamer = gamers::table.filter(gamers::name.eq(name))
            .filter(gamers::service.eq(service))
            .get_result::<Gamer>(&self.conn)
            .optional()?;

        if let Some(gamer) = gamer {
            Ok(gamer)
        } else {
            self.create_gamer(name, service)
        }
    }

    pub fn find_kifu_from_uid(&self, original_uid: &str) -> Result<Option<Kifu>, DatabaseError> {
        let k = kifu::table.filter(kifu::original_uid.eq(original_uid))
            .get_result::<Kifu>(&self.conn)
            .optional()?;

        Ok(k)
    }

    pub fn create_kifu(&self,
                       data: &str,
                       black: Option<&Gamer>,
                       white: Option<&Gamer>,
                       winner: Option<&Gamer>,
                       timestamp: Option<NaiveDateTime>,
                       original_uid: Option<&str>)
                       -> Result<Kifu, DatabaseError> {
        if let Some(original_uid) = original_uid {
            if let Some(kifu) = self.find_kifu_from_uid(original_uid)? {
                return Ok(kifu);
            }
        }
        let new_kifu = NewKifu {
            data: data,
            timestamp: timestamp,
            black_id: black.map(move |g| g.id),
            white_id: white.map(move |g| g.id),
            winner_id: winner.map(move |g| g.id),
            original_uid: original_uid,
        };

        match diesel::insert(&new_kifu)
            .into(kifu::table)
            .get_result(&self.conn) {
            Ok(kifu) => Ok(kifu),
            Err(e) => {
                Err(DatabaseError {
                    message: e.description().to_string(),
                    cause: None,
                })
            }
        }
    }

    pub fn find_analysis(&self, pos: &Position) -> Result<Option<Analysis>, DatabaseError> {
        let a = analysis::table.filter(analysis::engine.eq("Gikou"))
            .filter(analysis::position.eq(encoder::usi::sfen(pos)))
            .get_result::<Analysis>(&self.conn)
            .optional()?;

        Ok(a)
    }

    pub fn create_analysis(&self,
                           pos: &Position,
                           infos: &UsiEngineInfo)
                           -> Result<Analysis, Box<Error>> {
        let new_analysis = NewAnalysis {
            position: &encoder::usi::sfen(pos),
            engine: "Gikou",
            option: "",
            timestamp: Local::now().naive_local(),
            infos: &json::encode(infos)?,
        };

        Ok(diesel::insert(&new_analysis).into(analysis::table)
            .get_result(&self.conn)?)
    }
}

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    lazy_static! {
      static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
    }

    fn setup_conn_and_populate(d: &Database) {
        use schema::users;

        dotenv::dotenv().unwrap();
        diesel::delete(users::table).execute(&d.conn).unwrap();
        d.create_user("hoge@sample.com", "hoge").unwrap();
    }

    #[test]
    fn it_works() {
        let d = DATABASE.lock().unwrap();
        setup_conn_and_populate(&d);
        assert!(d.assume_user("hoge@sample.com", "hoge").is_ok());
        assert!(d.assume_user("fuga@sample.com", "hoge").is_err());
        assert!(d.assume_user("hoge@sample.com", "fuga").is_err());
    }

    #[test]
    fn duplicate_error() {
        let d = DATABASE.lock().unwrap();
        setup_conn_and_populate(&d);
        match d.create_user("hoge@sample.com", "hoge") {
            Ok(_) => panic!(),
            Err(e) => {
                /*
                match e.downcast::<diesel::result::Error>().unwrap() {
                    box diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => (),
                    _ => panic!(),
                }
                */
            }
        }
    }
}
