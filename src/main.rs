#[macro_use]
extern crate nom;
#[macro_use]
extern crate enum_primitive;

extern crate subprocess;
use subprocess::*;

mod types;
mod parser;
mod encoder;

fn main() {
    let mut p = Popen::create(&["ps", "x"],
                              PopenConfig { stdout: Redirection::Pipe, ..Default::default() });
}
