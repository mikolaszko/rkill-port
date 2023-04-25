use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let output = Command::new("netstat")
        .args(["-tunlp"])
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8(output.stdout).unwrap();
    if output_str.contains(port) {
        println!("{}", port);
        println!("port is in use");
    }
    let line = output_str
        .lines()
        .find(|line| line.contains(&format!(":{} ", port)))
        .expect("port not found in netstat output");
    let pid_row = line.split('/').nth(0).expect("Invalid PID");
    let pid = pid_row.split_at(pid_row.len() - 5);
    println!("Process listening on port {}: PID {:?}", port, pid.1);
    Command::new("kill")
        .args(["-9", pid.1])
        .spawn()
        .expect("failed to kill process");
    println!("Process killed on port {}", port);
}
