use crate::common::alias::{go, StderrHd, StdinHd, StdoutHd};
use crate::common::sync::{Ptr, Shared};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::{io, select};

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
            let mut w = writer.lock().await;
            let mut r = reader.lock().await;
            loop {
                let size = match r.read(&mut buf).await {
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
                if let Err(e) = w.write_all(&buf[..size]).await {
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
            let mut buf = [0u8; 32768];
            let mut reader = r.lock().await;
            let mut writer1 = w1.lock().await;
            let mut writer2 = w2.lock().await;
            while let Ok(u) = reader.read(&mut buf).await {
                if u == 0 {
                    break;
                }
                if writer1.write(&buf[..u]).await.is_err() {
                    break;
                }
                if writer2.write(&buf[..u]).await.is_err() {
                    break;
                }
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
                        res1=TransferStd::copy_byte(result,&mut *wer).await;
                    },
                    result = tr2.read_u8() => {
                        res2=TransferStd::copy_byte(result,&mut *wer).await;
                    }
                }
            }
        });
    }
}
pub struct TransferStd;
impl TransferStd {
    pub async fn copy_byte(b: io::Result<u8>, w: &mut (dyn AsyncWrite + Unpin + Send)) -> bool {
        if let Ok(u) = b {
            if w.write_u8(u).await.is_ok() {
                return true;
            }
        }
        false
    }
    pub async fn read_line(r: &mut (dyn AsyncRead + Unpin + Send)) -> String {
        let mut buf = Vec::new();
        while let Ok(u) = r.read_u8().await {
            if u == b'\n' {
                break;
            }
            buf.push(u);
        }
        String::from_utf8(buf).unwrap()
    }
}
pub trait MakeStdio {
    fn stdin(&self) -> String;
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;

    async fn make_stdio(&self) -> io::Result<(StdinHd, StdoutHd, StderrHd)> {
        let stdin = File::options().read(true).open(&self.stdin()).await?;
        let stdout = File::options()
            .write(true)
            .append(true)
            .open(&self.stdout())
            .await?;
        let stderr = File::options()
            .write(true)
            .append(true)
            .open(&self.stderr())
            .await?;
        Ok((Ptr::share(stdin), Ptr::share(stdout), Ptr::share(stderr)))
    }
}
