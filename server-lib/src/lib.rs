#[macro_use]
extern crate log;

extern crate env_logger;

#[macro_use]
extern crate json;

extern crate ws;

// iron crates
extern crate hyper;
extern crate iron;
extern crate logger;
extern crate mount;
extern crate router;
extern crate staticfile;

extern crate handlebars;
extern crate handlebars_iron as hbs;

extern crate core_lib;
extern crate database_lib;

mod users;
mod kifu;

use std::path::Path;

use logger::Logger;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;

use hbs::{HandlebarsEngine, DirectorySource};

use std::thread;

pub fn start_servers() {
    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    let mut mount = Mount::new();
    mount.mount("/get_moves", kifu::get_moves);
    mount.mount("/users", users::route);
    mount.mount("/", Static::new(Path::new("server-lib/dist")));

    let mut chain = Chain::new(mount);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(hbse);
    chain.link_after(logger_after);

    thread::spawn(move || {
        Iron::new(chain)
            .http(std::env::var("WEB_LISTEN").expect("WEB_LISTEN must be set"))
            .unwrap();
    });

    kifu::start_websock_server();
}
