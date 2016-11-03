use error::Result;

use std::net::SocketAddr;

use futures;
use futures::{Future};
use futures::stream::Stream;
use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;

pub fn run(listen_port: String, forward_to: String) -> Result<()> {
    let addr = format!("127.0.0.1:{}", listen_port)
        .parse::<SocketAddr>().unwrap();

    let fwd_addr = forward_to.parse::<SocketAddr>().unwrap();

    // Create the event loop that will drive this server
    let mut l = Core::new().unwrap();
    let handle = l.handle();

    // Create a TCP listener which will listen for incoming connections
    let accept_socket = TcpListener::bind(&addr, &handle).unwrap();

    let done = accept_socket.incoming().for_each(move |(server_socket, _addr)| {
        let client_pair = TcpStream::connect(&fwd_addr, &handle).map(|socket| socket.split());

        let pair = futures::lazy(move || futures::finished(server_socket.split()));
        let amt = pair.join(client_pair)
            .and_then(move |((server_reader, server_writer), (client_reader, client_writer))| {
                let upload = copy(server_reader, client_writer);
                let download = copy(client_reader, server_writer);
                upload.join(download)
            });

        let msg = amt.then(|res| {
            match res {
                Ok((sent, received)) => {
                    println!("bytes sent {:?}, bytes received {:?}", sent, received);
                    return Ok(());
                },
                Err(_) => Ok(()),
            }
        });

        handle.spawn(msg);

        Ok(())
    });

    l.run(done).unwrap();

    Ok(())
}