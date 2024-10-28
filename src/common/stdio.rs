use crate::common::sync::{Ptr, Shared};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::select;
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
            let mut buf_r1 = BufReader::new(r1.lock().await);
            let mut mem_r1 = String::new();
            let mut buf_r2 = BufReader::new(r2.lock().await);
            let mut mem_r2 = String::new();
            loop {
                select! {
                    result = buf_r1.read_line(&mut mem_r1) => {
                    let n = result.unwrap_or(0);
                    if n == 0 { break; }
                    w.write_all(mem_r1.as_bytes()).await.unwrap();
                    w.write_all(b"\n").await.unwrap();
                    mem_r1.clear();
                },

                // 从第二个流读取一行
                result = buf_r2.read_line(&mut mem_r2) => {
                    let n = result.unwrap_or(0);
                    if n == 0 { break; }
                    w.write_all(mem_r2.as_bytes()).await.unwrap();
                    w.write_all(b"\n").await.unwrap();
                    mem_r2.clear();
                }
                }
            }
        });
    }
}
