use error::Result;

use std::net::SocketAddr;

use futures;
use futures::Future;
use futures::stream::Stream;
use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;

pub fn run(listen_port: String, forward_port: String) -> Result<()> {

    let addr = format!("127.0.0.1:{}", listen_port)
        .parse::<SocketAddr>().unwrap();

    let fwd_addr = format!("127.0.0.1:{}", forward_port)
        .parse::<SocketAddr>().unwrap();

    // Create the event loop that will drive this server
    let mut l = Core::new().unwrap();
    let handle = l.handle();

    // Create a TCP listener which will listen for incoming connections
    let server_socket = TcpListener::bind(&addr, &handle).unwrap();

    let done = server_socket.incoming().for_each(move |(server_socker, _addr)| {
        let client_socket = TcpStream::connect(&fwd_addr, &handle);
        let client_pair = client_socket.map(|socket| socket.split());

        let pair = futures::lazy(move || futures::finished(server_socker.split()));
        let amt = pair.join(client_pair).and_then(move |((server_reader, server_writer), (client_reader, client_writer))| {
            copy(server_reader, client_writer).join(copy(client_reader, server_writer))
        });

        let msg = amt.map(|(sent, received)| println!("bytes sent {}, bytes received {}", sent, received)).map_err(|_| {});
        handle.spawn(msg);

        Ok(())
    });

    l.run(done).unwrap();

    Ok(())
}