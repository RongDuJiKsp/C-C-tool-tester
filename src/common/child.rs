use crate::common::stdio::TransferStdio;
use crate::common::sync::{Ptr, Shared};
use std::collections::HashMap;
use std::io::SeekFrom;
use std::process::Stdio;
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncRead, AsyncSeekExt, AsyncWrite};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};

pub fn run_exe_with_env(
    exe: &str,
    raw_args: &str,
    env: &HashMap<String, String>,
) -> io::Result<Child> {
    Command::new(exe)
        .args(
            &env.iter()
                .fold(raw_args.to_string(), |s, (from, to)| {
                    s.replace(&format!("&[{}]", from), to)
                })
                .split(" ")
                .filter(|x| *x != "")
                .collect::<Vec<_>>(),
        )
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub fn bind_client_to_files<
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
>(
    client: &mut Child,
    stdin: Shared<R>,
    stdout: Shared<W>,
    stderr: Shared<W>,
) {
    let i = client.stdin.take().expect("Stdin Is Err Can't Logger");
    TransferStdio::spawn_copy(Ptr::share(i), stdin);
    bind_client_output(client, stdout, stderr);
}

pub fn bind_client_output<W: AsyncWrite + Unpin + Send + 'static>(
    client: &mut Child,
    stdout: Shared<W>,
    stderr: Shared<W>,
) {
    let (o, e) = (
        client.stdout.take().expect("Stdout Is Err Can't Logger"),
        client.stderr.take().expect("Stderr Is Err Can't Logger"),
    );
    TransferStdio::spawn_copy(stdout, Ptr::share(o));
    TransferStdio::spawn_copy(stderr, Ptr::share(e));
}
pub fn client_streams(client: &mut Child) -> (ChildStdin, ChildStdout, ChildStderr) {
    (
        client.stdin.take().expect("Stdin Is Err Can't Logger"),
        client.stdout.take().expect("Stdout Is Err Can't Logger"),
        client.stderr.take().expect("Stderr Is Err Can't Logger"),
    )
}
pub async fn reset(f: &Shared<File>) {
    f.lock()
        .await
        .seek(SeekFrom::Start(0))
        .await
        .expect("Reset Stream failed");
}
