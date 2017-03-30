#[macro_use]
extern crate log;

extern crate env_logger;

extern crate regex;

#[macro_use]
extern crate json;

extern crate ws;

// iron crates
extern crate hyper;
extern crate hyper_native_tls;
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
mod scraping;

use std::path::Path;

use logger::Logger;

use iron::prelude::*;
use iron::status;
use staticfile::Static;
use mount::Mount;
use router::Router;

use handlebars_iron::{HandlebarsEngine, DirectorySource};

use std::thread;
use std::env;

use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;
use handlebars_iron::Template;

fn index(req: &mut Request) -> IronResult<Response> {
    use std::collections::BTreeMap;
    use rustc_serialize::json::{ToJson, Json};
    let mut data: BTreeMap<String, Json> = BTreeMap::new();
    if let Ok(s) = req.session().get::<users::Login>() {
        if s.is_some() {
            data.insert("login".to_string(), true.to_json());
        }
    }

    let mut resp = Response::new();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn start_servers() {
    env_logger::init().unwrap();
    let session_secret = env::var("SESSION_SECRET")
        .expect("SESSION_SECRET must be set")
        .as_bytes()
        .to_owned();
    let web_listen = env::var("WEB_LISTEN").expect("WEB_LISTEN must be set");


    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./server-lib/templates/", ".hbs")));
    hbse.reload().unwrap();

    let mut route = Router::new();
    route.get("/", index, "top");
    users::UserRoute::new(&mut route);
    kifu::KifuRoute::new(&mut route);

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("server-lib/app")));
    mount.mount("/", route);
    mount.mount("/bower_components",
                Static::new(Path::new("server-lib/bower_components")));

    let mut chain = Chain::new(mount);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(session_secret)));
    chain.link_after(error::ErrorReporter);
    chain.link_after(hbse);
    chain.link_after(logger_after);

    thread::spawn(move || {
        Iron::new(chain)
            .http(web_listen)
            .unwrap();
    });

    kifu::start_websock_server();
}
