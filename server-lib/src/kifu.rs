use std::str::FromStr;
use std::error::Error;

use rustc_serialize::json;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use router::Router;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::ContentType;

use core_lib::parser;
use core_lib::encoder;
use core_lib::types::*;

use database_lib;
use super::scraping;
use super::users;
use super::error::make_it_ironerror;

use bodyparser;
pub struct KifuRoute;

impl KifuRoute {
    pub fn new(router: &mut Router) -> KifuRoute {
        let prefix = "/kifu";
        router.get(format!("{}/", prefix), index, "kifu_index");
        router.get(format!("{}/:id", prefix), show, "kifu_show");
        router.post(format!("{}/fav/:id", prefix), fav, "kifu_fav");
        router.get(format!("{}/shougiwars/history/:user", prefix),
                   shougiwars_history,
                   "kifu_shougiwars_history");
        router.get(format!("{}/shougiwars/game/:game", prefix),
                   shougiwars_game,
                   "kifu_shougiwars_game");
        KifuRoute
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fav {
    pub fav: bool,
}

fn fav(req: &mut Request) -> IronResult<Response> {
    let id = {
        let router_ext = iexpect!(req.extensions.get::<Router>());
        let id = iexpect!(router_ext.find("id"));
        i32::from_str(id).map_err(make_it_ironerror)?
    };

    let fav = {
        let formdata = iexpect!(itry!(req.get::<bodyparser::Struct<Fav>>()));
        formdata.fav
    };
    info!("fav {} {}", id, fav);
    let mut d = database_lib::Database::new();
    let u = users::login_user(&mut d, req).map_err(make_it_ironerror)?;
    let k = d.get_kifu(id).unwrap();
    d.fav_kifu(&u, &k, fav).unwrap();
    Ok(Response::with((status::Found,
                       Redirect(url_for!(req, "kifu_show", "id" => id.to_string())))))
}

fn index(req: &mut Request) -> IronResult<Response> {
    use rustc_serialize::json::{ToJson, Object, Array};

    let mut d = database_lib::Database::new();
    let u = users::login_user(&mut d, req).map_err(make_it_ironerror)?;

    let mut kifu_data = Array::new();

    for kifu in d.list_kifu(&u).map_err(make_it_ironerror)? {
        let mut k = Object::new();
        k.insert("id".to_string(), kifu.id.to_json());

        fn id_to_gamer(d: &mut database_lib::Database,
                       id: Option<i32>)
                       -> Result<String, Box<Error>> {
            if let Some(id) = id {
                Ok(d.find_gamer(id)?.name)
            } else {
                Ok("".to_string())
            }
        }

        // TODO: fix 3n+1 problen
        let black = id_to_gamer(&mut d, kifu.black_id).map_err(make_it_ironerror)?;
        k.insert("black".to_string(), black.to_json());
        let white = id_to_gamer(&mut d, kifu.white_id).map_err(make_it_ironerror)?;
        k.insert("white".to_string(), white.to_json());
        let winner = id_to_gamer(&mut d, kifu.winner_id).map_err(make_it_ironerror)?;
        k.insert("winner".to_string(), winner.to_json());

        kifu_data.push(k.to_json());
    }

    let mut resp = Response::with((status::Ok, json::encode(&kifu_data).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

fn shougiwars_history(req: &mut Request) -> IronResult<Response> {
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let user = router_ext.find("user")
        .ok_or("parameter user not found".to_string())
        .map_err(make_it_ironerror)?;

    use rustc_serialize::json::{ToJson, Object, Array};
    let mut kifu_data = Array::new();

    let game_names = scraping::get_shougiwars_history(&user, 0).map_err(make_it_ironerror)?;
    for game_name in game_names {
        let mut k = Object::new();

        let game_info = scraping::get_shougiwars_info(&game_name).map_err(make_it_ironerror)?;

        k.insert("name".to_string(), game_name.to_json());
        k.insert("black".to_string(), game_info.black.to_json());
        k.insert("white".to_string(), game_info.white.to_json());
        k.insert("winner".to_string(), "".to_json());

        kifu_data.push(k.to_json());
    }

    let mut resp = Response::with((status::Ok, json::encode(&kifu_data).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)

}

fn show(req: &mut Request) -> IronResult<Response> {
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let id = iexpect!(router_ext.find("id"));
    let id = itry!(i32::from_str(id));

    let d = database_lib::Database::new();
    let k = itry!(d.get_kifu(id));
    let g = json::decode::<Game>(&k.data).unwrap();
    let moves = encoder::get_moves(&g).map_err(make_it_ironerror)?;

    let mut resp = Response::with((status::Ok, json::encode(&moves).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)
}

fn shougiwars_game(req: &mut Request) -> IronResult<Response> {
    let router_ext = iexpect!(req.extensions.get::<Router>());
    let game = iexpect!(router_ext.find("game"));

    let game_info = scraping::get_shougiwars_info(game).map_err(make_it_ironerror)?;
    let uid = format!("shougiwars:{}", game);
    let d = database_lib::Database::new();
    let black = itry!(d.create_or_find_gamer(&game_info.black, "shougiwars"));
    let white = itry!(d.create_or_find_gamer(&game_info.white, "shougiwars"));

    let k = {
        if let Some(k) = itry!(d.find_kifu_from_uid(&uid)) {
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
            itry!(d.create_kifu(data,
                                Some(&black),
                                Some(&white),
                                winner,
                                Some(game_info.timestamp),
                                Some(&uid)))
        }
    };

    use rustc_serialize::json::{ToJson, Object};
    let mut data = Object::new();
    data.insert("id".to_string(), k.id.to_json());

    let mut resp = Response::with((status::Ok, json::encode(&data).unwrap()));
    resp.headers.set(ContentType(Mime(TopLevel::Application,
                                      SubLevel::Json,
                                      vec![(Attr::Charset, Value::Utf8)])));
    Ok(resp)

}
