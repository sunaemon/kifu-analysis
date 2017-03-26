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

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &PgConnection, email: &str, password: &str) -> Result<User, Box<Error>> {
    use schema::users;

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

    let user: User = try!(diesel::insert(&new_user)
        .into(users::table)
        .get_result(conn));

    Ok(user)
}

pub fn verify_user(conn: &PgConnection, email: &str, password: &str) -> Result<(), Box<Error>> {
    use schema::users;

    let us = users::table.filter(users::email.eq(email))
        .load::<User>(conn)
        .expect("error loading user");

    if us.len() == 0 {
        return Err(Box::<Error>::from("No such user"));
    } else if us.len() > 1 {
        panic!("Unique validation goes wrong!! users: {:?}", us);
    }

    let user = &us[0];

    let mut hash: [u8; 512] = [0; 512];
    let mut mac = Hmac::new(Sha256::new(), password.as_bytes());
    crypto::pbkdf2::pbkdf2(&mut mac, &user.salt, 1000, &mut hash);

    if &user.hash[..] != &hash[..] {
        return Err(Box::<Error>::from("Wrong Password"));
    }

    Ok(())
}

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct Database {}

    lazy_static! {
      static ref LOCK: Mutex<Database> = Mutex::new(Database{});
    }

    fn setup_conn_and_populate(_d: &Database) -> PgConnection {
        use schema::users;

        dotenv::dotenv().unwrap();
        let conn = establish_connection();
        diesel::delete(users::table).execute(&conn).unwrap();
        create_user(&conn, "hoge@sample.com", "hoge").unwrap();
        conn
    }

    #[test]
    fn it_works() {
        let d = LOCK.lock().unwrap();
        let conn = setup_conn_and_populate(&d);
        assert!(verify_user(&conn, "hoge@sample.com", "hoge").is_ok());
        assert!(verify_user(&conn, "fuga@sample.com", "hoge").is_err());
        assert!(verify_user(&conn, "hoge@sample.com", "fuga").is_err());
    }

    #[test]
    fn duplicate_error() {
        let d = LOCK.lock().unwrap();
        let conn = setup_conn_and_populate(&d);
        match create_user(&conn, "hoge@sample.com", "hoge") {
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
