mod socket;

#[cfg(not(target_arch = "wasm32"))]
mod tls;

pub use socket::Socket;

#[cfg(not(target_arch = "wasm32"))]
pub use tls::{CertificateInput, MaybeTlsStream};

#[cfg(any(feature = "_rt-async-std", target_arch = "wasm32"))]
type PollReadBuf<'a> = [u8];

#[cfg(any(feature = "_rt-actix", feature = "_rt-tokio"))]
type PollReadBuf<'a> = sqlx_rt::ReadBuf<'a>;

#[cfg(any(feature = "_rt-async-std", target_arch = "wasm32"))]
type PollReadOut = usize;

#[cfg(any(feature = "_rt-actix", feature = "_rt-tokio"))]
type PollReadOut = ();
