extern crate server_lib;
extern crate dotenv;

fn main() {
    dotenv::dotenv().ok();
    server_lib::start_servers();
}
