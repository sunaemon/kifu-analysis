use std::io::Write;
use subprocess::*;
use types::*;
use super::parser;
use super::encoder;

pub struct UsiEngine {
    process: Popen,
}

impl UsiEngine {
    pub fn new() -> Self {
        UsiEngine {
            process: Popen::create(&["/home/sunaemon/Gikou/bin/release"],
                                   PopenConfig {
                                       stdin: Redirection::Pipe,
                                       stdout: Redirection::Pipe,
                                       stderr: Redirection::Pipe,
                                       ..Default::default()
                                   })
                .unwrap(),
        }
    }

    pub fn get_score(&self, pos: &Position, moves: &[Move], max_depth: u64) -> parser::usi::Score {
        let mut stdin_ref = self.process.stdin.as_ref().unwrap();
        let mut stdout_ref = self.process.stdout.as_ref().unwrap();
        //let mut stderr_ref = p.stderr.as_ref().unwrap();

        stdin_ref.write_all(b"isready\n").unwrap();

        let mut last_depth = 0;
        let mut last_score = parser::usi::Score::Cp(0);
        parser::usi::read_and_parse(&mut stdout_ref, |r| {
            if let parser::usi::Response::ReadyOk = r {
                let pos_string = encoder::usi::position(pos, moves);
                let pos = pos_string.as_bytes();

                stdin_ref.write(pos).unwrap();
                stdin_ref.write_all(b"\ngo\n").unwrap();
            } else if let parser::usi::Response::Infos(infos) = r {
                for info in infos {
                    if let parser::usi::Info::Depth(d) = info {
                        //println!("Depth: {}", d);

                        last_depth = d;
                    } else if let parser::usi::Info::Score(s) = info {
                        last_score = s;
                    }
                }

                if last_depth >= max_depth {
                    stdin_ref.write_all(b"stop\n").unwrap();
                }
            } else if let parser::usi::Response::BestMove(_) = r {
                return Some((last_score.clone()));
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
