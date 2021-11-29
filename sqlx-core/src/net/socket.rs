#![allow(dead_code)]

use std::io;
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(not(target_arch = "wasm32"))]
use sqlx_rt::{AsyncRead, AsyncWrite, TcpStream};

#[cfg(target_arch = "wasm32")]
use sqlx_rt::{AsyncRead, AsyncWrite, IoStream, WsMeta, WsStreamIo};
#[cfg(target_arch = "wasm32")]
type WSIoStream = IoStream<WsStreamIo, Vec<u8>>;

#[derive(Debug)]
pub enum Socket {
    #[cfg(not(target_arch = "wasm32"))]
    Tcp(TcpStream),

    #[cfg(all(unix, not(target_arch = "wasm32")))]
    Unix(sqlx_rt::UnixStream),

    #[cfg(target_arch = "wasm32")]
    WS((WsMeta, WSIoStream)),
}

impl Socket {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect_tcp(host: &str, port: u16) -> io::Result<Self> {
        TcpStream::connect((host, port)).await.map(Socket::Tcp)
    }

    #[cfg(all(unix, not(target_arch = "wasm32")))]
    pub async fn connect_uds(path: impl AsRef<Path>) -> io::Result<Self> {
        sqlx_rt::UnixStream::connect(path.as_ref())
            .await
            .map(Socket::Unix)
    }

    #[cfg(not(unix))]
    pub async fn connect_uds(_: impl AsRef<Path>) -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unix domain sockets are not supported outside Unix platforms.",
        ))
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn connect_ws(url: impl AsRef<str>) -> io::Result<Self> {
        WsMeta::connect(url, None)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "can't connect to ws stream"))
            .map(|(m, s)| Socket::WS((m, s.into_io())))
    }

    pub async fn shutdown(&mut self) -> io::Result<()> {
        #[cfg(all(feature = "_rt-async-std", not(target_arch = "wasm32")))]
        {
            use std::net::Shutdown;

            match self {
                Socket::Tcp(s) => s.shutdown(Shutdown::Both),

                #[cfg(all(unix, not(target_arch = "wasm32")))]
                Socket::Unix(s) => s.shutdown(Shutdown::Both),
            }
        }

        #[cfg(any(feature = "_rt-actix", feature = "_rt-tokio"))]
        {
            use sqlx_rt::AsyncWriteExt;

            match self {
                Socket::Tcp(s) => s.shutdown().await,

                #[cfg(unix)]
                Socket::Unix(s) => s.shutdown().await,
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let Socket::WS((m, _)) = self;
            m.close()
                .await
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "error closing ws stream"))
                .map(|_| ())
        }
    }
}

impl AsyncRead for Socket {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut super::PollReadBuf<'_>,
    ) -> Poll<io::Result<super::PollReadOut>> {
        match &mut *self {
            #[cfg(not(target_arch = "wasm32"))]
            Socket::Tcp(s) => Pin::new(s).poll_read(cx, buf),

            #[cfg(target_arch = "wasm32")]
            Socket::WS((_, s)) => Pin::new(s).poll_read(cx, buf),

            #[cfg(all(unix, not(target_arch = "wasm32")))]
            Socket::Unix(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for Socket {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            #[cfg(not(target_arch = "wasm32"))]
            Socket::Tcp(s) => Pin::new(s).poll_write(cx, buf),

            #[cfg(target_arch = "wasm32")]
            Socket::WS((_, s)) => Pin::new(s).poll_write(cx, buf),

            #[cfg(all(unix, not(target_arch = "wasm32")))]
            Socket::Unix(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            #[cfg(not(target_arch = "wasm32"))]
            Socket::Tcp(s) => Pin::new(s).poll_flush(cx),

            #[cfg(target_arch = "wasm32")]
            Socket::WS((_, s)) => Pin::new(s)
                .poll_flush(cx)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "error flushing ws stream")),

            #[cfg(all(unix, not(target_arch = "wasm32")))]
            Socket::Unix(s) => Pin::new(s).poll_flush(cx),
        }
    }

    #[cfg(any(feature = "_rt-actix", feature = "_rt-tokio"))]
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            #[cfg(not(target_arch = "wasm32"))]
            Socket::Tcp(s) => Pin::new(s).poll_shutdown(cx),

            #[cfg(all(unix, not(target_arch = "wasm32")))]
            Socket::Unix(s) => Pin::new(s).poll_shutdown(cx),
        }
    }

    #[cfg(all(feature = "_rt-async-std", not(target_arch = "wasm32")))]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            Socket::Tcp(s) => Pin::new(s).poll_close(cx),

            #[cfg(all(unix, not(target_arch = "wasm32")))]
            Socket::Unix(s) => Pin::new(s).poll_close(cx),
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            Socket::WS((_, s)) => Pin::new(s)
                .poll_close(cx)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "error closing ws stream")),

            #[cfg(all(unix, not(target_arch = "wasm32")))]
            Socket::Unix(s) => Pin::new(s).poll_close(cx),
        }
    }
}
