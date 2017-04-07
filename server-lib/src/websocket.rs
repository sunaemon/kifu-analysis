
use std::thread;
use std::env;
use std::str::FromStr;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use std::error::Error;

use rustc_serialize::json;

use core_lib::parser::usi::Info;
use core_lib::usi_engine;
use core_lib::types::*;

use ws;
use database_lib;

lazy_static! {
  static ref WEBSOCKET_LISTEN: String = env::var("WEBSOCKET_LISTEN").expect("WEBSOCKET_LISTEN must be set").to_string();
}

#[derive(PartialEq, Clone, RustcDecodable, RustcEncodable)]
struct InfoWithNum {
    n: usize,
    infos: Vec<Info>,
}

fn to_ws_err<T: Error>(e: T) -> ws::Error {
    ws::Error::new(ws::ErrorKind::Internal, e.description().to_owned())
}

struct HandlerData {
    kifu_id: Mutex<Option<i32>>,
    kifu_id_set: Condvar,
    to_terminate: Mutex<bool>,
}

struct Handler {
    data: Arc<HandlerData>,
    #[allow(dead_code)]
    handle: thread::JoinHandle<()>,
}

impl ws::Handler for Handler {
    fn on_message(&mut self, msg: ws::Message) -> Result<(), ws::Error> {
        if let ws::Message::Text(msg) = msg {
            debug!("Got message: {}", msg);
            let id = i32::from_str(&msg).map_err(to_ws_err)?;

            let mut kifu_id = self.data.kifu_id.lock().map_err(to_ws_err)?;
            *kifu_id = Some(id);
            self.data.kifu_id_set.notify_all();
        }
        Ok(())
    }
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        debug!("Connection closing due to ({:?}) {}", code, reason);
        *self.data.to_terminate.lock().unwrap() = true;
    }
}
impl Handler {
    fn new(out: ws::Sender) -> Handler {
        let data = Arc::new(HandlerData {
            kifu_id: Mutex::new(None),
            kifu_id_set: Condvar::new(),
            to_terminate: Mutex::new(false),
        });
        let data_copy = data.clone();

        let handle = thread::spawn(move || {
            let mut kifu_id = data.kifu_id.lock().unwrap();
            let ref kifu_id_set = data.kifu_id_set;

            while (*kifu_id).is_none() {
                kifu_id = kifu_id_set.wait(kifu_id).unwrap();
            }

            let kifu_id = (*kifu_id).unwrap();

            let d = database_lib::Database::new();
            let k = d.get_kifu(kifu_id).unwrap();

            let g = json::decode::<Game>(&k.data).unwrap();
            let en = usi_engine::UsiEngine::new();
            let max_depth = 20;

            for n in 0..(g.moves.len() + 1) {
                if *data.to_terminate.lock().unwrap() {
                    return;
                }

                let infos = en.get_score(&g.position,
                                         &g.moves[0..n],
                                         max_depth as u64,
                                         Duration::from_secs(1));

                let dat_to_send = json::encode(&(n, infos)).unwrap();

                info!("{}", dat_to_send);
                out.send(dat_to_send).unwrap();
            }
        });
        Handler {
            data: data_copy,
            handle: handle,
        }
    }
}

pub fn start_websock_server() {
    ws::listen(WEBSOCKET_LISTEN.clone(), Handler::new).unwrap();
}
