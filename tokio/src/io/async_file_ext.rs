use std::future::Future;
use crate::io::Result;

/// Async version of `std::os::unix::fs::FileExt`.
#[allow(async_fn_in_trait)]
pub trait AsyncFileExt {
    /// Read at a specific offset, asynchronously.
    async fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize>;
    /// Write at a specific offset, asynchronously.
    async fn write_at(&self, buf: &[u8], offset: u64) -> Result<usize>;
}
