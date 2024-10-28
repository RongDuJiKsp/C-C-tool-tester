use crate::common::alias::go;
use crate::common::stdio::TransferStd;
use crate::common::sync::Shared;
use crate::hacker::args::HackerArgs;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt};
use crate::common::strings::StringPkg;

const PATTEN_FLAG: &str = "&[client]";

#[derive(Clone)]
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
    fn read_news_and_use_it(&self, sender: &mut Vec<String>, recv: String) {
        if let Some(val) = StringPkg::extract_value(&recv, &self.arg.line_expr_of_new_client, PATTEN_FLAG) {
            sender.push(self.arg.line_expr_of_use_client.replace(PATTEN_FLAG, &val));
        }
    }
    fn start_listen_new_client_and_use<
        W: AsyncWrite + Send + Unpin + 'static,
        R: AsyncRead + Send + Unpin + 'static,
    >(
        &self,
        r: Shared<R>,
        w: Shared<W>,
    ) {
        let d = self.clone();
        go(async move {
            let mut sender_buf = Vec::new();
            loop {
                let mut red = r.lock().await;
                let recv = TransferStd::read_line(&mut *red).await;
                if recv.is_empty() {
                    break;
                }
                drop(red);
                d.read_news_and_use_it(&mut sender_buf, recv);
                let mut w = w.lock().await;
                for s in &sender_buf {
                    if w.write(s.as_bytes()).await.is_err() {
                        break;
                    };
                    if w.write_u8(b'\n').await.is_err() {
                        break;
                    }
                }
                drop(w);
                sender_buf.clear()
            }
        });
    }
    fn start_timeout_exec_command<W: AsyncWrite + Send + Unpin + 'static>(&self, w: Shared<W>) {}
}
