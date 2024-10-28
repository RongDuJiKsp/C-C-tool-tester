use crate::common::alias::{StderrHd, StdinHd, StdoutHd};
use crate::common::sync::Ptr;
use clap::Parser;
use tokio::fs::File;
use tokio::io;

#[derive(Debug, Parser, Clone)]
pub struct HackerArgs {
    #[arg(short, long)]
    pub exe: String,
    #[arg(short, long = "args")]
    pub args_raw: String,
    #[arg(long = "exp-new-cl", help = "stdout with name &[client]")]
    pub line_expr_of_new_client: String,
    #[arg(long = "exp-use-cl", help = "run command with name &[client]")]
    pub line_expr_of_use_client: String,
    #[arg(long)]
    pub stdin: String,
    #[arg(long)]
    pub stdout: String,
    #[arg(long)]
    pub stderr: String,
    #[arg(long = "cycle")]
    pub cycle_cmds_raw: String,
    #[arg(long = "c_time")]
    pub cycle_cmds_time: u64,
}
impl HackerArgs {
    pub fn cycle_cmds(&self) -> Vec<String> {
        self.cycle_cmds_raw
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
    pub fn args(&self) -> Vec<String> {
        self.args_raw
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
    pub async fn make_stdio(&self) -> io::Result<(StdinHd, StdoutHd, StderrHd)> {
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
