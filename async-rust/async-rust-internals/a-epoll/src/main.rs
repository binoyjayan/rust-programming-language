//! Main program
use std::io;
use std::{io::Write, net::TcpStream};

mod ffi;
mod poll;

use poll::Poll;

fn get_req(path: &str) -> Vec<u8> {
    format!(
        "GET {} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        path
    )
    .into_bytes()
}

fn main() -> io::Result<()> {
    let poll = Poll::new()?;
    let n_events = 5;
    let mut streams = Vec::new();
    let addr = "localhost:8080";
    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{}/request-{}", delay, i);
        let request = get_req(&url_path);
        let mut stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(&request)?;
        poll.registry()
            .register(&stream, i as usize, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }
    Ok(())
}
