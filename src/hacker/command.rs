use crate::common::alias::go;
use crate::common::stdio::TransferStd;
use crate::common::sync::Shared;
use crate::hacker::args::HackerArgs;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt};
pub struct CommandCtx {
    arg: HackerArgs,
}
impl CommandCtx {
    pub fn make(arg: HackerArgs) -> Self {
        Self { arg }
    }
    pub fn send_commend_and_waiting<
        W: AsyncWrite + Send + Unpin + 'static,
        R: AsyncRead + Send + Unpin + 'static,
    >(
        &self,
        sender: Shared<W>,
        recv: Shared<R>,
    ) {
        self.start_listen_new_client_and_use(recv.clone(), sender.clone());
        self.start_timeout_exec_command(sender.clone());
    }
    async fn async_send_cmd_and_waiting(&self, sender: &mut Vec<String>, recv: String) {

    }
    fn start_listen_new_client_and_use<
        W: AsyncWrite + Send + Unpin + 'static,
        R: AsyncRead + Send + Unpin + 'static,
    >(
        &self,
        r: Shared<R>,
        w: Shared<W>,
    ) {
        go(async move {
            let mut sender_buf = Vec::new();
            loop {
                let mut w = w.lock().await;
                let mut red = r.lock().await;
                let recv = TransferStd::read_line(&mut *red).await;
                self.async_send_cmd_and_waiting(&mut sender_buf, recv).await;
                for s in &sender_buf {
                    let _ = w.write(s.as_bytes()).await;
                    let _ = w.write_u8(b'\n').await;
                }
                sender_buf.clear()
            }
        });
    }
    fn start_timeout_exec_command<W: AsyncWrite + Send + Unpin + 'static>(&self, w: Shared<W>) {}
}
