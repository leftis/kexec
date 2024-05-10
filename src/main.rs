use std::io::Error;
use std::process::{Child, ChildStdin, Command, Output, Stdio,};

const K_ARGS: [&str; 3] = ["get", "pods", "--output=name"];
const FZF_ARGS: [&str; 2] = ["--height=60%", "--reverse"];

fn fzf_cmd() -> Command {
    Command::new("fzf")
}
fn k_cmd() -> Command {
    Command::new("kubectl")
}
fn fzf(args: std::slice::Iter<&str>) -> Result<Child, Error> {
    fzf_cmd().args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
}
fn push_pods(args: std::slice::Iter<&str>, stdin: ChildStdin) -> Output {
    k_cmd().args(args).stdout(stdin).output().unwrap()
}
fn run_shell(pod: &str, cmd: &str) -> Child {
    k_cmd().args(["exec", "-it", pod, "--", cmd]).spawn().expect("some error")
}

fn main() {
    let mut fzf = fzf(FZF_ARGS.iter()).expect("with error");
    push_pods(K_ARGS.iter(), fzf.stdin.take().unwrap());
    let results = fzf.wait_with_output().unwrap();

    if let Some(0) = results.status.code() {
        let pod = &String::from_utf8_lossy(&results.stdout).replace("\n", "");
        let cmd_status = run_shell(pod, "/bin/bash").wait();
        match cmd_status {
            Ok(st) => println!("Exited with status {:?}", st),
            Err(err) => println!("Errored out with {:?}", err)
        }
    }
}

