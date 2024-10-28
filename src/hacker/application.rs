use crate::common;
use crate::common::alias::{go, StderrHd, StdinHd, StdoutHd};
use crate::common::child::{
    bind_client_output, bind_client_to_files, client_streams, reset, run_exe_with_env,
};
use crate::common::sync::{Ptr, Shared};
use crate::hacker::command::send_commend_and_waiting;
use clap::Parser;
use std::collections::HashMap;
use std::env;
use std::io::SeekFrom;
use std::str::SplitWhitespace;
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncRead, AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use tokio::process::Child;
use crate::common::stdio::TransferStdio;

#[derive(Debug, Parser)]
struct HackerArgs {
    #[arg(short, long)]
    exe: String,
    #[arg(short, long = "args")]
    args_raw: String,
    #[arg(long = "exp-new-cl", help = "stdout with name &[client]")]
    line_expr_of_new_client: String,
    #[arg(long = "exp-use-cl", help = "run command with name &[client]")]
    line_expr_of_use_client: String,
    #[arg(long)]
    stdin: String,
    #[arg(long)]
    stdout: String,
    #[arg(long)]
    stderr: String,
    #[arg(long = "cycle")]
    cycle_cmds_raw: String,
}
impl HackerArgs {
    fn cycle_cmds(&self) -> Vec<String> {
        self.cycle_cmds_raw
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
    fn args(&self) -> Vec<String> {
        self.args_raw
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
    async fn make_stdio(&self) -> io::Result<(StdinHd, StdoutHd, StderrHd)> {
        let stdin = File::options().read(true).open(&self.stdin).await?;
        let stdout = File::options()
            .write(true)
            .append(true)
            .open(&self.stdout)
            .await?;
        let stderr = File::options()
            .write(true)
            .append(true)
            .open(&self.stderr)
            .await?;
        Ok((Ptr::share(stdin), Ptr::share(stdout), Ptr::share(stderr)))
    }
}

pub async fn app() {
    let arg = HackerArgs::parse_from(env::args().skip(2));
    let (s_in, s_out, s_err) = arg.make_stdio().await.expect("Can't Load Stdio:");
    loop {
        let mut server = run_exe_with_env(&arg.exe, &arg.args_raw, &HashMap::new())
            .expect("Can't Start Server,Panic ing");
        reset(&s_in).await;
        //stdio stream
        let (i_stream, o_stream, err_stream) = client_streams(&mut server);
        let (i_stream, o_stream, err_stream) = (Ptr::share(i_stream), Ptr::share(o_stream), Ptr::share(err_stream));
        //stdin
        TransferStdio::spawn_copy(i_stream.clone(), s_in.clone());
        //stdout
        let (stdout_reader, stdout_writer) = tokio_pipe::pipe().expect("Create Pipe Failed");
        TransferStdio::copy_many(o_stream.clone(), Ptr::share(stdout_writer), s_out.clone());
        //stderr
        let (stderr_reader, stderr_writer) = tokio_pipe::pipe().expect("Create Pipe Failed");
        TransferStdio::copy_many(err_stream.clone(), Ptr::share(stderr_writer), s_err.clone());
        //union stdout and stderr
        let (std_union_reader, std_union_writer) = tokio_pipe::pipe().expect("Create Pipe Failed");
        TransferStdio::union(Ptr::share(std_union_writer), Ptr::share(stdout_reader), Ptr::share(stderr_reader));
        //handle
        send_commend_and_waiting(i_stream.clone(), Ptr::share(std_union_reader));
        //wait exit;
        let _ = server.wait().await;
    }
}
