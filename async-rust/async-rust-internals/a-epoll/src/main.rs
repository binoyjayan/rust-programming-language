//! Main program
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

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
    let mut poll = Poll::new()?; // needs to be mutable for poll(&mut self,...)
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
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    let mut completed = 0;
    // Reuse events buffer instead of reallocating every loop.
    let mut events: Vec<ffi::Event> = Vec::with_capacity(10);
    while completed < n_events {
        // safe before poll repopulates
        events.clear();
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("SPURIOUS WAKEUP");
            continue;
        }
        completed += handle_events(&events, &mut streams)?;
    }
    println!("FINISHED");
    Ok(())
}

fn handle_events(events: &[ffi::Event], streams: &mut [TcpStream]) -> io::Result<usize> {
    let mut finished = 0;
    for event in events {
        let idx = event.token();
        let mut buf = vec![0u8; 4096];
        loop {
            // Cannot use read_to_end because it will not allow
            // us a chance to handle 'WouldBlock' error.
            match streams[idx].read(&mut buf) {
                Ok(0) => {
                    // EOF
                    finished += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&buf[..n]);
                    println!("RECEIVED[{}]:\n{}\n----------\n", idx, txt);
                    continue;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Data transfer is not complete but the data is not ready now
                    // so break and wait for the next event.
                    break;
                }
                Err(e) => return Err(e),
            }
        }
    }
    Ok(finished)
}
