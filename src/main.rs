use std::io::Error;
use std::process::{
    Child, 
    ChildStdin, 
    Command, 
    Stdio,
};
fn k_cmd() -> Command {
    Command::new("kubectl")
}

fn fzf() -> Result<Child, Error> {
    Command::new("fzf")
        .args(["--height=60%", "--reverse"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
}

fn run_shell(pod: &str, cmd: &str) -> Result<Child, Error> {
    k_cmd()
        .args(["exec", "-it", pod, "--", cmd])
        .spawn()
}

fn push_pods(stdin: ChildStdin) {
    let _ = k_cmd()
                .args(["get", "pods", "--output=name"])
                .stdout(stdin)
                .output();
}
fn main() {
    let mut fzf = fzf().expect("Fzf not responding");
    
    push_pods(fzf.stdin.take().unwrap());
    let results = fzf.wait_with_output().unwrap();

    if let Some(0) = results.status.code() {
        let pod = &String::from_utf8_lossy(&results.stdout).replace("\n", "");
        let cmd_status = run_shell(pod, "/bin/bash").expect("Panicked!").wait();
        match cmd_status {
            Ok(st) => println!("Exited with status {:?}", st),
            Err(err) => println!("Errored out with {:?}", err)
        }
    }
}

