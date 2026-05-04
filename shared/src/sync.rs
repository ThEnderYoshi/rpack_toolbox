//! Defines some helpers related to asynchronous logic.

use tokio::task::JoinHandle;

pub async fn flatten<T>(handle: JoinHandle<crate::Result<T>>) -> crate::Result<T> {
    handle.await?
}
