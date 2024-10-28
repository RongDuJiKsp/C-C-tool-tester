use crate::common::child::{bind_client_to_files, reset, run_exe_with_env};
use crate::common::stdio::MakeStdio;
use clap::Parser;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Parser)]
struct ChickenArgs {
    #[arg(short, long)]
    pub exe: String,
    #[arg(short, long = "args")]
    pub args_raw: String,
    #[arg(long)]
    pub stdin: String,
    #[arg(long)]
    pub stdout: String,
    #[arg(long)]
    pub stderr: String,
}
impl MakeStdio for ChickenArgs {
    fn stdin(&self) -> String {
        self.stdin.clone()
    }

    fn stdout(&self) -> String {
        self.stdout.clone()
    }

    fn stderr(&self) -> String {
        self.stderr.clone()
    }
}

pub async fn app() {
    let arg = ChickenArgs::parse_from(env::args().skip(2));
    let (s_in, s_out, s_err) = arg.make_stdio().await.expect("Can't Load Stdio:");
    loop {
        let mut client = run_exe_with_env(&arg.exe, &arg.args_raw, &HashMap::new())
            .expect("Can't Start Client,Panic ing");
        reset(&s_in).await;
        bind_client_to_files(&mut client, s_in.clone(), s_out.clone(), s_err.clone());
        //wait exit;
        let _ = client.wait().await;
    }
}
