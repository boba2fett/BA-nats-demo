use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::BufMut;
use futures::ready;
use tokio::io::{self, ReadBuf};

use futures::stream::Stream;

pub struct StreamReader<S> {
    pub stream: S,
    pub buffer: Vec<u8>,
}

impl<S> io::AsyncRead for StreamReader<S>
where
    S: Stream<Item = Vec<u8>> + Unpin,
{
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        let outstanding = buf.remaining().min(self.buffer.len());
        if outstanding > 0 {
            buf.put_slice(&self.buffer[..outstanding]);
            self.buffer.drain(..outstanding);
            return Poll::Ready(Ok(()));
        }
        let stream = Pin::new(&mut self.stream);
        let chunk = match ready!(stream.poll_next(cx)) {
            Some(chunk) => chunk,
            None => return Poll::Ready(Ok(())),
        };

        let len = buf.remaining().min(chunk.len());
        buf.put_slice(&chunk[..len]);
        self.buffer = chunk[len..].to_vec();

        Poll::Ready(Ok(()))
    }
}

pub struct VecStream {
    data: Vec<u8>,
    pos: usize,
}

impl VecStream {
    pub fn new(data: Vec<u8>) -> Self {
        VecStream {
            data,
            pos: 0,
        }
    }
}

impl Stream for VecStream {
    type Item = u8;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;
        if this.pos < this.data.len() {
            let byte = this.data[this.pos];
            this.pos += 1;
            Poll::Ready(Some(byte))
        } else {
            Poll::Ready(None)
        }
    }
}

pub struct VecStreamReader<S> {
    pub stream: S,
}

impl<S> io::AsyncRead for VecStreamReader<S>
where
    S: Stream<Item = u8> + Unpin,
{
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        let mut stream = Pin::new(&mut self.stream);
        let len = buf.remaining();
        for _ in 0..len {
            let chunk = match ready!(stream.as_mut().poll_next(cx)) {
                Some(chunk) => chunk,
                None => return Poll::Ready(Ok(())),
            };
            buf.put_u8(chunk);
        }
        Poll::Ready(Ok(()))
    }
}

pub struct VecReader {
    pub vec: Vec<u8>,
}

impl io::AsyncRead for VecReader {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        let outstanding = buf.remaining().min(self.vec.len());
        if outstanding > 0 {
            buf.put_slice(&self.vec[..outstanding]);
            self.vec.drain(..outstanding);
            return Poll::Ready(Ok(()));
        }
        Poll::Ready(Ok(()))
    }
}