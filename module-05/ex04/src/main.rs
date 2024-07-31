use std::io;
use std::io::Write;
use std::process::{Child, Command, Stdio};



fn main() {
    let mut commands: Vec<Vec<String>> = Vec::new();

    commands.push(Vec::new());
    for arg in std::env::args().skip(1) {
        if arg == "," {
            if commands.last().unwrap().is_empty() {
                println!("empty command");
                return;
            }
            commands.push(Vec::new());
        } else {
            commands.last_mut().unwrap().push(arg);
        }
    }

    let mut processes: Vec<(Vec<String>, Child)> = Vec::new();
    for c in commands {
        let mut args = c.iter();
        let name = args.next().unwrap();
        match Command::new(name)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(process) => processes.push((c, process)),
            Err(err) => {
                println!("{name}: {err}");
            }
        }
    }

    match write_output(processes) {
        Ok(()) => (),
        Err(err) => println!("{err}"),
    }
}

fn write_output(mut processes: Vec<(Vec<String>, Child)>) -> io::Result<()> {
    let mut stdout = std::io::stdout();
    while !processes.is_empty() {
        if let Some(found) = processes
            .iter_mut()
            .position(|(_, process)| process.try_wait().ok().flatten().is_some())
        {
            let (c, process) = processes.swap_remove(found);
            let output = process.wait_with_output()?;
            write!(stdout, "===== ")?;
            for arg in c {
                write!(stdout, "{arg} ")?;
            }
            writeln!(stdout, " =====")?;
            stdout.write_all(&output.stdout)?;
            writeln!(stdout)?;
        }
    }
    Ok(())
}
