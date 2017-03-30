use std::thread;
use std::env;

use router::Router;
use core_lib::parser;
use core_lib::encoder;
use core_lib::usi_engine;
use core_lib::types::*;

use iron::prelude::*;
use iron::status;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::ContentType;
use iron::Url;
use handlebars_iron::Template;
use urlencoded::UrlEncodedBody;

use ws;
use json;
use database_lib;
use super::scraping;

const KIFU: &'static str =
    "+7776FU,L600	-3334FU,L599	+2726FU,L600	-8384FU,L596	+2625FU,L599	-4132KI,L593	+6978KI,L597	\
     -8485FU,L591	+2524FU,L596	-2324FU,L588	+2824HI,L596	-8586FU,L584	+8786FU,L595	-8286HI,L582	\
     +2434HI,L592	-0027FU,L572	+8822UM,L586	-3122GI,L568	+3432RY,L584	-6152KI,L539	+0075KA,L531	\
     -8689RY,L533	+7553UM,L525	-5253KI,L526	+3222RY,L500	-0086KA,L517	+5948OU,L493	-8664KA,L499	\
     +0042GI,L425	-5161OU,L475	+4253NG,L420	-6453KA,L474	+0051KI,L413	-6151OU,L468	+0052KI,L412	\
     SENTE_WIN_CHECKMATE";

pub fn start_websock_server() {
    ws::listen(env::var("WEBSOCKET_LISTEN").expect("WEBSOCKET_LISTEN must be set"), |out| {
            thread::spawn(move || {
              let g = parser::shougiwars::parse(KIFU.as_bytes()).unwrap();
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

pub struct KifuRoute;

impl KifuRoute {
    pub fn new(router: &mut Router) -> KifuRoute {
        let prefix = "/kifu";
        router.get(format!("{}/get_move/:id", prefix),
                   get_move,
                   "kifu_get_move");
        router.get(format!("{}/:id", prefix), show, "kifu_show");
        router.get(format!("{}/new", prefix), render_new, "kifu_render_new");
        router.get(format!("{}/", prefix), render_index, "kifu_render_index");
        router.get(format!("{}/get_move/:id", prefix), get_move, "kifu_new");
        router.get(format!("{}/shougiwars/history/:user", prefix),
                   render_shougiwars_history,
                   "kifu_render_shougiwars_history");
        router.get(format!("{}/shougiwars/game/:game", prefix),
                   render_shougiwars_game,
                   "kifu_render_shougiwars_game");
        KifuRoute
    }
}

fn show(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/show", ())).set_mut(status::Ok);
    Ok(resp)
}

fn render_new(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/new", ())).set_mut(status::Ok);
    Ok(resp)
}

fn render_index(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/index", ())).set_mut(status::Ok);
    Ok(resp)
}

fn render_shougiwars_history(req: &mut Request) -> IronResult<Response> {
    let user = req.extensions.get::<Router>().unwrap().find("user").unwrap();

    use rustc_serialize::json::{ToJson, Object, Array};
    let mut data = Object::new();

    match scraping::scrape_shougiwars_history(include_bytes!("../test/history")) {
        //match scraping::get_shougiwars_history(&user, 0) {
        Ok(game_names) => {
            let mut games: Array = Array::new();
            for game_name in game_names {
                let mut game: Object = Object::new();
                game.insert("name".to_string(), game_name.to_json());
                let url = url_for!(req, "kifu_render_shougiwars_game", "game" => game_name);
                game.insert("url".to_string(), url.to_string().to_json());
                games.push(game.to_json());
            }
            data.insert("games".to_string(), games.to_json());
        }
        Err(e) => {
            return Err(IronError::new(scraping::ScrapingError::General(e.description()
                                          .to_string()),
                                      status::BadRequest))
        }
    };

    info!("{:?}", data);

    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/shougiwars/history", data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn get_move(_req: &mut Request) -> IronResult<Response> {
    let g = parser::shougiwars::parse(KIFU.as_bytes()).unwrap();
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

    let mut resp = Response::with((status::Ok, json::JsonValue::Array(ret).dump()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}



fn render_shougiwars_game(req: &mut Request) -> IronResult<Response> {
    let game = req.extensions.get::<Router>().unwrap().find("game").unwrap();

    use rustc_serialize::json::{ToJson, Json, Object};
    let mut data = Object::new();

    match scraping::scrape_shougiwars_game(include_bytes!("../test/game")) {
        //match scraping::get_shougiwars_game(game) {
        Ok(kifu_data) => {
            let g = parser::shougiwars::parse(kifu_data.as_bytes()).unwrap();
            let mut p = Position::hirate();

            let kifu = Array::new();

            let m = Object::new();
            m.insert("move".to_string(), Json::Null);
            m.insert("move_str".to_string(), Json::Null);
            m.insert("position".to_string(), encoder::json::position(&p));

            data.insert("kifu".to_string(), kifu.to_json())
        }
        Err(e) => {
            return Err(IronError::new(scraping::ScrapingError::General(e.description()
                                          .to_string()),
                                      status::BadRequest))
        }
    };

    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/shougiwars/game", data)).set_mut(status::Ok);
    Ok(resp)

}
