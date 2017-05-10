use error::Result;

use std::net::SocketAddr;

use futures::{Future};
use futures::stream::Stream;
use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;

pub fn run(listen_port: String, forward_to: String) -> Result<()> {
    let listen_addr = try!(format!("0.0.0.0:{}", listen_port).parse::<SocketAddr>());
    let fwd_addr = try!(forward_to.parse::<SocketAddr>());

    // Create the event loop that will drive this server
    let mut l = try!(Core::new());
    let handle = l.handle();

    // Create a TCP listener which will listen for incoming connections
    let accept_socket = try!(TcpListener::bind(&listen_addr, &handle));

    let done = accept_socket.incoming().for_each(move |(server_socket, _addr)| {
        let client_pair = TcpStream::connect(&fwd_addr, &handle).map(|socket| socket.split());

        // For up-to date and correct version, see
        // official example at https://github.com/tokio-rs/tokio-core/blob/master/examples/proxy.rs

        let amt = client_pair
            .and_then(move |(client_reader, client_writer)| {
                let (server_reader, server_writer) = server_socket.split();
                let upload = copy(server_reader, client_writer);
                let download = copy(client_reader, server_writer);
                upload.join(download)
            });

        let msg = amt.then(|res| {
            if let Ok((sent, received)) = res {
                println!("bytes sent {:?}, bytes received {:?}", sent, received);
            }
            Ok(())
        });

        handle.spawn(msg);

        Ok(())
    });

    try!(l.run(done));

    Ok(())
}
