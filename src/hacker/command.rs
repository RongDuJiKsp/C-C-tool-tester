use tokio::io::{AsyncRead, AsyncWrite, BufReader};
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
        let mut sender_buf = Vec::new();
        let mut reader_buf = String::new();
        let r = recv.lock().await;
        loop {}
    });
}

async fn async_send_cmd_and_waiting(
    sender: &Vec<String>,
    recv: String,
) {}