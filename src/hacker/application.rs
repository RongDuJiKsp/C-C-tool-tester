use std::str::SplitWhitespace;
use clap::Parser;
use tokio::fs::File;
use tokio::io;
use crate::common::alias::{StderrHd, StdinHd, StdoutHd};
use crate::common::sync::Ptr;

#[derive(Debug, Parser)]
struct HackerArgs {
    __side: String,
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
        self.cycle_cmds_raw.split_whitespace().map(|x| x.to_string()).collect()
    }
    fn args(&self) -> Vec<String> {
        self.args_raw.split_whitespace().map(|x| x.to_string()).collect()
    }
    async fn make_stdio(&self) -> io::Result<(StdinHd, StdoutHd, StderrHd)> {
        let stdin = File::options().read(true).open(&self.stdin).await?;
        let stdout = File::options().write(true).append(true).open(&self.stdout).await?;
        let stderr = File::options().write(true).append(true).open(&self.stderr).await?;
        Ok((Ptr::share(stdin), Ptr::share(stdout), Ptr::share(stderr)))
    }
}
pub async fn app() {}