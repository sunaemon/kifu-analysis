use std::env;
use std::str::FromStr;

use rustc_serialize::json;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use router::Router;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::ContentType;
use handlebars_iron::Template;

use core_lib::parser;
use core_lib::encoder;
use core_lib::types::*;

use database_lib;
use super::scraping;
use super::users;
use super::error::make_it_ironerror;

lazy_static! {
  static ref WEBSOCKET_URL: String = env::var("WEBSOCKET_URL").expect("WEBSOCKET_URL must be set").to_string();
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
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let id = iexpect!(router_ext.find("id"));
    let id = i32::from_str(id).map_err(make_it_ironerror)?;
    use rustc_serialize::json::{ToJson, Object};
    let mut data = Object::new();

    data.insert("kifu".to_string(), id.to_json());
    data.insert("websocket_url".to_string(), (*WEBSOCKET_URL).to_json());

    let mut resp = Response::new();
    resp.set_mut(Template::new("kifu/show", data)).set_mut(status::Ok);
    Ok(resp)
}

fn own(req: &mut Request) -> IronResult<Response> {
    let id = {
        let router_ext = iexpect!(req.extensions.get::<Router>());
        let id = iexpect!(router_ext.find("id"));
        i32::from_str(id).map_err(make_it_ironerror)?
    };
    let mut d = database_lib::Database::new();
    let u = users::login_user(&mut d, req).map_err(make_it_ironerror)?;
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
    let u = users::login_user(&mut d, req).map_err(make_it_ironerror)?;

    let mut data = Object::new();
    let mut kifu_data = Array::new();

    for kifu in d.list_kifu(&u).map_err(make_it_ironerror)? {
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
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let user = router_ext.find("user")
        .ok_or("parameter user not found".to_string())
        .map_err(make_it_ironerror)?;

    use rustc_serialize::json::{ToJson, Object, Array};
    let mut data = Object::new();

    let game_names = scraping::get_shougiwars_history(&user, 0).map_err(make_it_ironerror)?;
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

fn show_moves(req: &mut Request) -> IronResult<Response> {
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let id = iexpect!(router_ext.find("id"));
    let id = i32::from_str(id).map_err(make_it_ironerror)?;

    let d = database_lib::Database::new();
    let k = d.get_kifu(id).map_err(make_it_ironerror)?;
    let g = json::decode::<Game>(&k.data).unwrap();
    let moves = encoder::get_moves(&g).map_err(make_it_ironerror)?;

    let mut resp = Response::with((status::Ok, json::encode(&moves).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

fn render_shougiwars_game(req: &mut Request) -> IronResult<Response> {
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let game = iexpect!(router_ext.find("game"));

    let game_info = scraping::get_shougiwars_info(game).map_err(make_it_ironerror)?;
    let uid = format!("shougiwars:{}", game);
    let d = database_lib::Database::new();
    let black = d.create_or_find_gamer(&game_info.black, "shougiwars").map_err(make_it_ironerror)?;
    let white = d.create_or_find_gamer(&game_info.white, "shougiwars").map_err(make_it_ironerror)?;

    let k = {
        if let Some(k) = d.find_kifu_from_uid(&uid).map_err(make_it_ironerror)? {
            info!("i know {}", uid);
            k
        } else {
            info!("fetching {}", uid);
            let kifu_data = scraping::get_shougiwars_game(game).map_err(make_it_ironerror)?;
            let g = parser::shougiwars::parse(kifu_data.as_bytes()).map_err(make_it_ironerror)?;
            let data = &json::encode(&g).map_err(make_it_ironerror)?;
            let winner = match g.issue {
                Some(i) => {
                    match i {
                        IssueOfGame::Win(c, _) => {
                            match c {
                                Color::Black => Some(&black),
                                Color::White => Some(&white),
                            }
                        }
                        _ => None,
                    }
                }
                None => None,
            };
            d.create_kifu(data,
                             Some(&black),
                             Some(&white),
                             winner,
                             Some(game_info.timestamp),
                             Some(&uid))
                .map_err(make_it_ironerror)?
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
