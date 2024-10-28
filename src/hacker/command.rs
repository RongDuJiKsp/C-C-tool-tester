use tokio::io::{AsyncRead, AsyncWrite};
use crate::common::alias::go;
use crate::common::sync::Shared;

pub fn send_commend_and_waiting<
    W: AsyncWrite + Send + Unpin + 'static,
    R: AsyncRead + Send + Unpin + 'static,
>(
    sender: Shared<W>,
    recv: Shared<R>,
) {
    go(async move {
        async_send_cmd_and_waiting(sender, recv).await;
    });
}

async fn async_send_cmd_and_waiting<
    W: AsyncWrite + Send + Unpin + 'static,
    R: AsyncRead + Send + Unpin + 'static,
>(
    sender: Shared<W>,
    recv: Shared<R>,
) {
    
}