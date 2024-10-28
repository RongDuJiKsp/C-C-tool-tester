use crate::common::sync::{Ptr, Shared};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::{io, select};
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
    pub fn union<
        W: AsyncWrite + Send + Unpin + 'static,
        R1: AsyncRead + Send + Unpin + 'static,
        R2: AsyncRead + Send + Unpin + 'static,
    >(
        w: Shared<W>,
        r1: Shared<R1>,
        r2: Shared<R2>,
    ) {
        go(async move {
            let mut tr1 = r1.lock().await;
            let mut tr2 = r2.lock().await;
            let mut wer = w.lock().await;
            let (mut res1, mut res2) = (true, true);
            while res1 || res2 {
                select! {
                    result = tr1.read_u8() => {
                        res1=copy_byte(result,&mut *wer).await;
                    },
                    result = tr2.read_u8() => {
                        res2=copy_byte(result,&mut *wer).await;
                    }
                }
            }
        });
    }
}
async fn copy_byte(b: io::Result<u8>, w: &mut dyn AsyncWrite + Unpin + Send) -> bool {
    if let Ok(u) = b {
        if w.write_u8(u).await.is_ok() {
            return true;
        }
    }
    false
}
