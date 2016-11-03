extern crate clap;
extern crate futures;
extern crate tokio_core;
extern crate env_logger;
#[macro_use]
extern crate log;

use clap::{Arg, App};

mod server;
mod error;

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("Mio Proxy")
        .version("1.0")
        .author("Nerijus Arlauskas <nercury@gmail.com>")
        .about("Proxies service from local port to another port")
        .arg(Arg::with_name("listen")
            .short("i")
            .long("listen")
            .help("sets the port to listen to")
            .multiple(false)
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("forward")
            .short("o")
            .long("forward")
            .help("sets the port to forward to on localhost")
            .multiple(false)
            .takes_value(true)
            .required(true))
        .get_matches();

    let listen_port = matches.value_of("listen").expect("failed to get listen port");
    let forward_port = matches.value_of("forward").expect("failed to get forward port");

    server::run(listen_port.to_string(), forward_port.to_string())
        .expect("server error!");
}