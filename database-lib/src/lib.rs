#![feature(box_patterns)]

// diesel crates
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate rand;
extern crate crypto;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use models::{User, NewUser};
use rand::{Rng, OsRng};

use crypto::hmac::Hmac;
use crypto::sha2::Sha256;

use std::error::Error;
use std::env;
use std::fmt;

use schema::users;

pub struct Database {
    conn: PgConnection,
}

#[derive(Debug, Clone)]
pub struct DatabaseError {
    message: String,
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

impl Database {
    pub fn new() -> Database {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        Database { conn: conn }
    }

    pub fn create_user(self: &Database,
                       email: &str,
                       password: &str)
                       -> Result<User, DatabaseError> {
        let mut mac = Hmac::new(Sha256::new(), password.as_bytes());
        let mut rng = OsRng::new().unwrap();
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

        match diesel::insert(&new_user)
            .into(users::table)
            .get_result(&self.conn) {
            Ok(user) => Ok(user),
            Err(e) => Err(DatabaseError { message: e.description().to_string() }),
        }

    }


    pub fn verify_user(self: &Database, email: &str, password: &str) -> Result<(), DatabaseError> {
        let us = users::table.filter(users::email.eq(email))
            .load::<User>(&self.conn)
            .expect("error loading user");

        if us.len() == 0 {
            return Err(DatabaseError { message: "No such user".to_string() });
        } else if us.len() > 1 {
            panic!("Unique validation goes wrong!! users: {:?}", us);
        }

        let user = &us[0];

        let mut hash: [u8; 512] = [0; 512];
        let mut mac = Hmac::new(Sha256::new(), password.as_bytes());
        crypto::pbkdf2::pbkdf2(&mut mac, &user.salt, 1000, &mut hash);

        if &user.hash[..] != &hash[..] {
            return Err(DatabaseError { message: "Wrong Password".to_string() });
        }

        Ok(())
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
        assert!(d.verify_user("hoge@sample.com", "hoge").is_ok());
        assert!(d.verify_user("fuga@sample.com", "hoge").is_err());
        assert!(d.verify_user("hoge@sample.com", "fuga").is_err());
    }

    #[test]
    fn duplicate_error() {
        let d = DATABASE.lock().unwrap();
        setup_conn_and_populate(&d);
        match d.create_user("hoge@sample.com", "hoge") {
            Ok(_) => panic!(),
            Err(e) => {
                match e.downcast::<diesel::result::Error>().unwrap() {
                    box diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => (),
                    _ => panic!(),
                }
            }
        }
    }
}
