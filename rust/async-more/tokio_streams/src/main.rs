use tokio::fs::File;
use tokio::io::{AsyncReadExt, ReadBuf};
use bytes::{BytesMut, Buf};
use std::pin::Pin;
use std::io;
use std::env;

pub struct FileStream {
    file: File,
    buffer: [u8; 1024],
    leftover: BytesMut,
}

impl FileStream {
    pub async fn new(file_path: &str) -> io::Result<FileStream> {
        let file = File::open(file_path).await?;
        Ok(FileStream {
            file,
            buffer: [0; 1024],
            leftover: BytesMut::new(),
        })
    }

    pub async fn next_line(&mut self) -> io::Result<Option<String>> {
        loop {
            if let Some(position) = self.leftover.iter().position(|&b| b == b'\n') {
                let line = self.leftover.split_to(position);
                self.leftover.advance(1); // skip the newline
                let line = String::from_utf8(line.to_vec()).expect("Invalid UTF-8");
                return Ok(Some(line));
            }

            let mut read_buf = ReadBuf::new(&mut self.buffer);
            match Pin::new(&mut self.file).read_buf(&mut read_buf).await {
                Ok(0) => return Ok(None), // EOF
                Ok(_) => {
                    self.leftover.extend_from_slice(read_buf.filled());
                }
                Err(err) => return Err(err),
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        return Ok(());
    }
    let file_path = &args[1];

    let mut lines = FileStream::new(file_path).await?;
    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }
    Ok(())
}