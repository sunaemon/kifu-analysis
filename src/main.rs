extern crate subprocess;
use subprocess::*;

fn main() {
    let mut p = Popen::create(&["ps", "x"],
                              PopenConfig { stdout: Redirection::Pipe, ..Default::default() });
}
