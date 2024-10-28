use crate::common::child::{client_streams, reset, run_exe_with_env};
use crate::common::stdio::TransferStdio;
use crate::common::sync::Ptr;
use crate::hacker::command::{CommandCtx};
use clap::Parser;
use std::collections::HashMap;
use std::env;
use crate::hacker::args::HackerArgs;

pub async fn app() {
    let arg = HackerArgs::parse_from(env::args().skip(2));
    let (s_in, s_out, s_err) = arg.make_stdio().await.expect("Can't Load Stdio:");
    loop {
        let mut server = run_exe_with_env(&arg.exe, &arg.args_raw, &HashMap::new())
            .expect("Can't Start Server,Panic ing");
        reset(&s_in).await;
        //stdio stream
        let (i_stream, o_stream, err_stream) = client_streams(&mut server);
        let (i_stream, o_stream, err_stream) = (
            Ptr::share(i_stream),
            Ptr::share(o_stream),
            Ptr::share(err_stream),
        );
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
        TransferStdio::union(
            Ptr::share(std_union_writer),
            Ptr::share(stdout_reader),
            Ptr::share(stderr_reader),
        );
        //handle
        CommandCtx::make(arg.clone()).send_commend_and_waiting(i_stream.clone(), Ptr::share(std_union_reader));
        //wait exit;
        let _ = server.wait().await;
    }
}
