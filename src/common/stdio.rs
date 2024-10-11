use crate::common::sync::Shared;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

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
}
