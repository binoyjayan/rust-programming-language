use futures::stream::Stream;
use futures::task::{Context, Poll};
use std::pin::Pin;
use tokio::io::{AsyncRead, ReadBuf};

pub struct ReadStream<R: AsyncRead + Unpin> {
    reader: R,
    buf: [u8; 1024],
}

impl<R: AsyncRead + Unpin> ReadStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: [0; 1024],
        }
    }
}

impl<R: AsyncRead + Unpin> Stream for ReadStream<R> {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let up = self.get_mut();
        let reader = Pin::new(&mut up.reader);
        let mut buf = ReadBuf::new(&mut up.buf);

        match reader.poll_read(cx, &mut buf) {
            Poll::Ready(Ok(())) if buf.filled().is_empty() => Poll::Ready(None),
            Poll::Ready(Ok(())) => {
                let s = String::from_utf8_lossy(buf.filled()).to_string();
                Poll::Ready(Some(s))
            }
            Poll::Ready(Err(_)) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
