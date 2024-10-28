use crate::common::alias::go;
use crate::common::sync::Shared;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt};
use crate::common::stdio::TransferStd;
use crate::hacker::args::HackerArgs;
pub struct CommandCtx {
    arg: HackerArgs,
}
impl CommandCtx {
    pub fn make(arg: HackerArgs) -> Self {
        Self {
            arg
        }
    }
    pub fn send_commend_and_waiting<
        W: AsyncWrite + Send + Unpin + 'static,
        R: AsyncRead + Send + Unpin + 'static,
    >(&self,
      sender: Shared<W>,
      recv: Shared<R>,
    ) {
        go(async move {
            let mut sender_buf = Vec::new();
            let r = recv.lock().await;
            let mut w = sender.lock().await;
            loop {
                let recv = TransferStd::read_line(&mut *r).await;
                self.async_send_cmd_and_waiting(&mut sender_buf, recv).await;
                for s in &sender_buf {
                    let _ = w.write(s.as_bytes()).await;
                    let _ = w.write_u8(b'\n').await;
                }
                sender_buf.clear()
            }
        });
    }
    async fn async_send_cmd_and_waiting(&self, sender: &mut Vec<String>, recv: String) {

    }
}



