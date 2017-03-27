#[macro_use]
extern crate log;

extern crate env_logger;

#[macro_use]
extern crate json;

extern crate ws;

// iron crates
extern crate hyper;
#[macro_use]
extern crate iron;
extern crate logger;
extern crate mount;
#[macro_use]
extern crate router;
extern crate staticfile;

extern crate handlebars;
extern crate handlebars_iron;

extern crate iron_sessionstorage;
extern crate urlencoded;

extern crate url;

extern crate rustc_serialize;

extern crate core_lib;
extern crate database_lib;

mod users;
mod kifu;
mod error;

use std::path::Path;

use logger::Logger;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;

use handlebars_iron::{HandlebarsEngine, DirectorySource};

use std::thread;
use std::env;

use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

pub fn start_servers() {
    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./server-lib/templates/", ".hbs")));

    let users_route = users::UserRoute::new();

    let mut mount = Mount::new();
    mount.mount("/get_moves", kifu::get_moves);
    mount.mount("/users", users_route);
    mount.mount("/", Static::new(Path::new("server-lib/dist")));

    let mut chain = Chain::new(mount);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(env::var("SESSION_SECRET")
        .expect("SESSION_SECRET must be set")
        .as_bytes()
        .to_owned())));
    chain.link_after(error::ErrorReporter);
    chain.link_after(hbse);
    chain.link_after(logger_after);

    thread::spawn(move || {
        Iron::new(chain)
            .http(env::var("WEB_LISTEN").expect("WEB_LISTEN must be set"))
            .unwrap();
    });

    kifu::start_websock_server();
}
