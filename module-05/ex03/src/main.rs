use std::os::unix::process::CommandExt;
use std::process::Command;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let command = match args.next() {
        Some(arg) => arg,
        None => {
            println!("missing command.");
            return;
        }
    };

    let mut lines = Vec::new();
    for i in std::io::stdin().lines() {
        match i {
            Ok(line) => lines.push(line),
            Err(err) => {
                println!("stdin: {err}");
                return;
            }
        }
    }

    let err = Command::new(command).args(args).args(lines).exec();
    println!("exec: {err}");
}
