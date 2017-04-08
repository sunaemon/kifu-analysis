extern crate server_lib;
extern crate dotenv;
extern crate daemonize;
extern crate clap;

fn main() {
    dotenv::dotenv().ok();
    server_lib::start_servers();
}
