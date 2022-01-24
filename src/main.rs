use std::io::{
    Write,
    stdout
};
use std::env;

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
    }
}
