#[macro_use]
extern crate log;

extern crate env_logger;

extern crate dotenv;

#[macro_use]
extern crate json;

extern crate ws;

// iron crates
extern crate hyper;
extern crate iron;
extern crate logger;
extern crate mount;
extern crate staticfile;

extern crate core_lib;
extern crate database_lib;

use std::path::Path;
use iron::prelude::*;
use logger::Logger;
use staticfile::Static;
use mount::Mount;

use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use iron::status::Status;

use core_lib::types::*;
use core_lib::parser;
use core_lib::encoder;
use core_lib::usi_engine;

const KIFU: &'static str =
    "+7776FU,L600	-3334FU,L599	+2726FU,L600	-8384FU,L596	+2625FU,L599	-4132KI,L593	+6978KI,L597	\
     -8485FU,L591	+2524FU,L596	-2324FU,L588	+2824HI,L596	-8586FU,L584	+8786FU,L595	-8286HI,L582	\
     +2434HI,L592	-0027FU,L572	+8822UM,L586	-3122GI,L568	+3432RY,L584	-6152KI,L539	+0075KA,L531	\
     -8689RY,L533	+7553UM,L525	-5253KI,L526	+3222RY,L500	-0086KA,L517	+5948OU,L493	-8664KA,L499	\
     +0042GI,L425	-5161OU,L475	+4253NG,L420	-6453KA,L474	+0051KI,L413	-6151OU,L468	+0052KI,L412	\
     SENTE_WIN_CHECKMATE";

fn get_moves(_req: &mut Request) -> IronResult<Response> {
    let g = parser::shougi_wars::parse(KIFU.as_bytes()).unwrap();
    let mut p = Position::hirate();

    let mut ret = Vec::new();
    ret.push(object! {
                   "move" => json::JsonValue::Null,
                   "move_str" => json::JsonValue::Null,
                   "position" => encoder::json::position(&p)
             });
    for m in g.moves.iter() {
        p.make_move(m).unwrap();
        ret.push(object! {
                   "move" => encoder::json::enc_move(&m),
                   "move_str" => encoder::japanese::enc_move(&m),
                   "position" => encoder::json::position(&p)
                 });
    }


    let mut resp = Response::with((Status::Ok, json::JsonValue::Array(ret).dump()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

fn main() {
    use std::thread;

    dotenv::dotenv().ok();

    env_logger::init().unwrap();

    let mut mount = Mount::new();
    mount.mount("/get_moves", get_moves);
    mount.mount("/", Static::new(Path::new("dist")));

    let mut chain = Chain::new(mount);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    thread::spawn(move || { Iron::new(chain).http("localhost:3000").unwrap(); });

    ws::listen("localhost:3001", |out| {
            thread::spawn(move || {
              let g = parser::shougi_wars::parse(KIFU.as_bytes()).unwrap();
              let en = usi_engine::UsiEngine::new();
              let d = 20;

              for n in 0 .. (g.moves.len() + 1) {
                let s = object! {
                     "n" => n,
                     "score" => encoder::json::score(&en.get_score(&g.position, &g.moves[0..n], d as u64))}.dump();

                info!("{}", s);
                out.send(s).unwrap();
              }
            });

            move |msg| {
                println!("Got message: {}", msg);
                Ok(())
            }
        })
        .unwrap()
}
