use std::io::{
    Write,
    stdout
};
use std::env;
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{
    fork,
    getpid,
    getppid
};

fn main() {
    let mut line = String::new();
    let b1 = std::io::stdin();
    let cwd = env::current_dir().unwrap()
        .as_path().display().to_string();
    loop {
        print!("$$ {} ", cwd);
        stdout().flush().unwrap();
        b1.read_line(&mut line).unwrap();
        println!("So you entered {}", line);
        line = String::new();
    }
}
