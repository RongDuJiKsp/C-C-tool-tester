use crate::common::sync::{Ptr, Shared};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use crate::common::alias::go;

pub struct TransferStdio;
impl TransferStdio {
    pub fn spawn_copy<
        W: AsyncWrite + Unpin + Send + 'static,
        R: AsyncRead + Unpin + Send + 'static,
    >(
        writer: Shared<W>,
        reader: Shared<R>,
    ) {
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let size = match reader.lock().await.read(&mut buf).await {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Read ToWrite.file Occupied Error:{}", e);
                        break;
                    }
                };
                if size == 0 {
                    println!("Copying Close");
                    break;
                }
                if let Err(e) = writer.lock().await.write_all(&buf[..size]).await {
                    println!("Write Stdin Occupied Error :{}", e);
                    break;
                }
            }
        });
    }
    pub fn copy_many<
        W1: AsyncWrite + Send + Unpin + 'static,
        W2: AsyncWrite + Send + Unpin + 'static,
        R: AsyncRead + Send + Unpin + 'static,
    >(
        r: Shared<R>,
        w1: Shared<W1>,
        w2: Shared<W2>,
    ) {
        go(async move {
            let mut buf = Ptr::rw_share([0u8; 32768]);
            while let Ok(u) = r.lock().await.read(&mut *buf.write().await).await {
                if u == 0 {
                    break;
                }
                let (mut wh1, mut wh2) = (w1.lock().await, w2.lock().await);
                let (mut rh1, mut rh2) = (buf.read().await, buf.read().await);
                let hd1 = go(async move {
                    wh1.write(&rh1)
                });
                let hd2 = go(async move {});
            }
        });
    }
}
