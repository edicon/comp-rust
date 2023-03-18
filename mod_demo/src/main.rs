mod main_mod;

use mod_lib::network;
use mod_lib::server;

fn main() {
    println!("main: Hello, world!");
    main_mod::run();
    network::connect();
    server::run();
}
