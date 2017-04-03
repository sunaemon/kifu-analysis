use std::thread;
use std::env;
use std::str::FromStr;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;

use rustc_serialize::json;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use router::Router;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::ContentType;
use handlebars_iron::Template;

use core_lib::parser;
use core_lib::parser::usi::Score;
use core_lib::encoder;
use core_lib::usi_engine;
use core_lib::types::*;

use ws;
use database_lib;
use super::scraping;
use super::users;

lazy_static! {
  static ref WEBSOCKET_LISTEN: String = env::var("WEBSOCKET_LISTEN").expect("WEBSOCKET_LISTEN must be set").to_string();
  static ref WEBSOCKET_URL: String = env::var("WEBSOCKET_URL").expect("WEBSOCKET_URL must be set").to_string();
}

#[derive(PartialEq, Clone, RustcDecodable, RustcEncodable)]
struct ScoreWithNum {
    n: usize,
    score: Score,
}

pub fn start_websock_server() {
    ws::listen(WEBSOCKET_LISTEN.clone(), |out| {
            let kifu_id_recv = Arc::new((Mutex::new(None), Condvar::new()));
            let kifu_id_send = kifu_id_recv.clone();
            thread::spawn(move || {
                let &(ref kifu_id, ref kifu_id_updated) = &*kifu_id_send;
                let mut kifu_id = kifu_id.lock().unwrap();

                while (*kifu_id).is_none() {
                    kifu_id = kifu_id_updated.wait(kifu_id).unwrap();
                }

                let kifu_id = (*kifu_id).unwrap();

                let d = database_lib::Database::new();
                let k = d.get_kifu(kifu_id).unwrap();

                let g = json::decode::<Game>(&k.data).unwrap();
                let en = usi_engine::UsiEngine::new();
                let d = 20;

                for n in 0..(g.moves.len() + 1) {
                    let s = ScoreWithNum {
                        n: n,
                        score: en.get_score(&g.position,
                                            &g.moves[0..n],
                                            d as u64,
                                            Duration::from_secs(3)),
                    };
                    let dat_to_send = json::encode(&s).unwrap();

                    info!("{}", dat_to_send);
                    out.send(dat_to_send).unwrap();
                }
            });

            move |msg| {
                if let ws::Message::Text(msg) = msg {
                    println!("Got message: {}", msg);
                    let id = i32::from_str(&msg).unwrap();
                    let &(ref kifu_id, ref kifu_id_updated) = &*kifu_id_recv;
                    let mut kifu_id = kifu_id.lock().unwrap();
                    *kifu_id = Some(id);
                    kifu_id_updated.notify_all();
                }
                Ok(())
            }
        })
        .unwrap()
}

pub struct KifuRoute;

impl KifuRoute {
    pub fn new(router: &mut Router) -> KifuRoute {
        let prefix = "/kifu";
        router.get(format!("{}/:id", prefix), show, "kifu_show");
        router.get(format!("{}/own/:id", prefix), own, "kifu_own");
        router.get(format!("{}/new", prefix), render_new, "kifu_render_new");
        router.get(format!("{}/", prefix), render_index, "kifu_render_index");
        router.get(format!("{}/shougiwars/history/:user", prefix),
                   render_shougiwars_history,
                   "kifu_render_shougiwars_history");
        router.get(format!("{}/shougiwars/game/:game", prefix),
                   render_shougiwars_game,
                   "kifu_render_shougiwars_game");
        router.get(format!("{}/show_moves/:id", prefix),
                   show_moves,
                   "kifu_get_move");
        KifuRoute
    }
}

fn show(req: &mut Request) -> IronResult<Response> {
    let id = i32::from_str(req.extensions.get::<Router>().unwrap().find("id").unwrap()).unwrap();
    use rustc_serialize::json::{ToJson, Object};
    let mut data = Object::new();

    data.insert("kifu".to_string(), id.to_json());
    data.insert("websocket_url".to_string(), (*WEBSOCKET_URL).to_json());

    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/show", data)).set_mut(status::Ok);
    Ok(resp)
}

fn own(req: &mut Request) -> IronResult<Response> {
    let id = i32::from_str(req.extensions.get::<Router>().unwrap().find("id").unwrap()).unwrap();
    let mut d = database_lib::Database::new();
    let u = users::login_user(&mut d, req).unwrap();
    let k = d.get_kifu(id).unwrap();
    d.own_kifu(&u, &k).unwrap();
    Ok(Response::with((status::Found,
                       Redirect(url_for!(req, "kifu_show", "id" => id.to_string())))))
}

fn render_new(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/new", ())).set_mut(status::Ok);
    Ok(resp)
}

fn render_index(req: &mut Request) -> IronResult<Response> {
    use rustc_serialize::json::{ToJson, Object, Array};

    let mut d = database_lib::Database::new();
    let u = users::login_user(&mut d, req).unwrap();

    let mut data = Object::new();
    let mut kifu_data = Array::new();

    for kifu in d.list_kifu(&u).unwrap() {
        let mut k = Object::new();
        let url = url_for!(req, "kifu_show", "id" => kifu.id.to_string());
        k.insert("url".to_string(), url.to_string().to_json());
        k.insert("id".to_string(), kifu.id.to_json());

        kifu_data.push(k.to_json());
    }

    data.insert("kifu".to_string(), kifu_data.to_json());

    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/index", data)).set_mut(status::Ok);
    Ok(resp)
}

fn render_shougiwars_history(req: &mut Request) -> IronResult<Response> {
    let user = req.extensions.get::<Router>().unwrap().find("user").unwrap();

    use rustc_serialize::json::{ToJson, Object, Array};
    let mut data = Object::new();

    let game_names = iwtry!(scraping::get_shougiwars_history(&user, 0));
    let mut games: Array = Array::new();
    for game_name in game_names {
        let mut game: Object = Object::new();
        game.insert("name".to_string(), game_name.to_json());
        let url = url_for!(req, "kifu_render_shougiwars_game", "game" => game_name);
        game.insert("url".to_string(), url.to_string().to_json());
        games.push(game.to_json());
    }
    data.insert("games".to_string(), games.to_json());

    info!("{:?}", data);

    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/shougiwars/history", data)).set_mut(status::Ok);
    Ok(resp)
}


#[derive(PartialEq, Clone, RustcDecodable, RustcEncodable)]
struct Movement {
    movement: Option<Move>,
    movestr: Option<String>,
    position: Position,
}

fn get_moves(g: &Game) -> Vec<Movement> {
    let mut p = Position::hirate();
    let mut kifu = Vec::new();
    kifu.push(Movement {
        movement: None,
        movestr: None,
        position: p.clone(),
    });
    for m in g.moves.iter() {
        p.make_move(m).unwrap();
        kifu.push(Movement {
            movement: Some(m.clone()),
            movestr: Some(encoder::japanese::enc_move(m)),
            position: p.clone(),
        });
    }

    kifu
}

fn show_moves(req: &mut Request) -> IronResult<Response> {
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let id = iexpect!(router_ext.find("id"));
    let id = iwtry!(i32::from_str(id));

    let d = database_lib::Database::new();
    let k = iwtry!(d.get_kifu(id));
    let g = json::decode::<Game>(&k.data).unwrap();
    let moves = get_moves(&g);

    let mut resp = Response::with((status::Ok, json::encode(&moves).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

fn render_shougiwars_game(req: &mut Request) -> IronResult<Response> {
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let game = iexpect!(router_ext.find("game"));

    let d = database_lib::Database::new();
    let uid = format!("shougiwars:{}", game);

    let k = {
        if let Some(k) = d.find_kifu_from_uid(&uid) {
            info!("i know {}", uid);
            k
        } else {
            info!("fetching {}", uid);
            let kifu_data = iwtry!(scraping::get_shougiwars_game(game));
            let g = iwtry!(parser::shougiwars::parse(kifu_data.as_bytes()));
            iwtry!(d.create_kifu(&iwtry!(json::encode(&g)), None, None, None, Some(&uid)))
        }
    };


    use rustc_serialize::json::{ToJson, Object};
    let mut data = Object::new();
    data.insert("kifu".to_string(), k.id.to_json());
    data.insert("websocket_url".to_string(), (*WEBSOCKET_URL).to_json());
    data.insert("import_url".to_string(),
                url_for!(req, "kifu_own", "id" => k.id.to_string())
                    .to_string()
                    .to_json());

    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/shougiwars/game", data)).set_mut(status::Ok);
    Ok(resp)
}
