extern crate subprocess;
use subprocess::*;
mod lib;

fn main() {
    let mut p = Popen::create(&["ps", "x"],
                              PopenConfig { stdout: Redirection::Pipe, ..Default::default() });
}
