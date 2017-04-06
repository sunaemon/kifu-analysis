use std::io::{Write, BufReader};
use std::env;
use std::time::{Duration, Instant};

use subprocess::*;

use super::parser;
use super::encoder;
use types::*;

pub struct UsiEngine {
    process: Popen,
}


#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct UsiEngineInfo {
    depth: u64,
    score: parser::usi::Score,
    moves: Vec<encoder::Movement>,
}

impl UsiEngine {
    pub fn new() -> Self {
        let mut work_dir = env::home_dir().unwrap();
        work_dir.push("Gikou/bin");
        let script = format!("cd {}; ./release", work_dir.to_str().unwrap());

        debug!("run {}", script);

        UsiEngine {
            process: Popen::create(&["/bin/bash", "-c", &script],
                                   PopenConfig {
                                       stdin: Redirection::Pipe,
                                       stdout: Redirection::Pipe,
                                       stderr: Redirection::Pipe,
                                       ..Default::default()
                                   })
                .unwrap(),
        }
    }

    pub fn get_score(&self,
                     pos: &Position,
                     moves: &[Move],
                     max_depth: u64,
                     max_time: Duration)
                     -> UsiEngineInfo {
        let mut stdin_ref = self.process.stdin.as_ref().unwrap();
        let stdout_ref = self.process.stdout.as_ref().unwrap();
        //let mut stderr_ref = p.stderr.as_ref().unwrap();

        let start = Instant::now();

        stdin_ref.write_all(b"isready\n").unwrap();

        let mut last_depth = 0;
        let mut last_score = parser::usi::Score::Cp(0);
        let mut last_pv = Vec::new();

        parser::usi::read_and_parse(&mut BufReader::new(stdout_ref), |r| {
            if let parser::usi::Response::ReadyOk = r {
                let pos_string = encoder::usi::position(pos, moves);
                let pos = pos_string.as_bytes();

                info!("ready ok at: {:?}", start.elapsed());

                stdin_ref.write_all(pos).unwrap();
                stdin_ref.write_all(b"\ngo\n").unwrap();
            } else if let parser::usi::Response::Infos(infos) = r {
                for info in infos {
                    if let parser::usi::Info::Depth(d) = info {
                        last_depth = d;
                    } else if let parser::usi::Info::Score(s) = info {
                        last_score = s;
                    } else if let parser::usi::Info::Pv(pv) = info {
                        last_pv = pv;
                    }
                }

                if last_depth >= max_depth || start.elapsed() > max_time {
                    info!("stop at: {:?}", start.elapsed());
                    stdin_ref.write_all(b"stop\n").unwrap();
                }
            } else if let parser::usi::Response::BestMove(_) = r {
                info!("best move at: {:?}", start.elapsed());

                let mut p = Position::hirate();
                let mut movements = Vec::new();

                if let Some(m) = moves.last() {
                    movements.push(encoder::Movement {
                        movement: Some(m.clone()),
                        movestr: Some(encoder::japanese::enc_move(&m)),
                        position: p.clone(),
                    })
                } else {
                    movements.push(encoder::Movement {
                        movement: None,
                        movestr: None,
                        position: p.clone(),
                    })
                }

                for pm in last_pv.clone() {
                    let m = pm.to_move(&p);
                    p.make_move(&m).unwrap();
                    movements.push(encoder::Movement {
                        movement: Some(m),
                        movestr: Some(encoder::japanese::enc_move(&m)),
                        position: p.clone(),
                    });
                }

                return Some(UsiEngineInfo {
                    depth: last_depth,
                    score: last_score.clone(),
                    moves: movements,
                });
            }
            None
        })
    }
}

impl Drop for UsiEngine {
    fn drop(&mut self) {
        let mut stdin_ref = self.process.stdin.as_ref().unwrap();
        stdin_ref.write_all(b"quit\n").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let en = UsiEngine::new();
        let info = en.get_score(&Position::hirate(), &[], 10, Duration::from_secs(3));
        match info.score {
            parser::usi::Score::Mate(_) => panic!(),
            parser::usi::Score::Cp(p) => assert!(p > 0 && p < 200),
        }
    }
}
