use std::fs;
use std::io;
use std::path::Path;

struct Size {
    value: u64,
}

impl Size {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn display(&self) {
        print!("\x1b[1A\x1b[2K");
        match self.value {
            0..=999 => println!("{} bytes", self.value),
            1000..=999999 => println!("{:.1} kilobytes", (self.value as f32) / 1000.0),
            1000000..=999999999 => println!("{:.1} megabytes", (self.value as f32) / 1000000.0),
            _ if self.value > 1000000000 => {
                println!("{:.1} gigabytes", (self.value as f32) / 1000000000.0)
            }
            _ => todo!(),
        }
    }
}

fn read_recursive(base_dir: &Path, size: &mut Size) -> io::Result<()> {
    if base_dir.is_dir() {
        for entry in fs::read_dir(base_dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            size.value += metadata.len() as u64;
            size.display();
            if path.is_dir() {
                read_recursive(&path, size)?;
            }
        }
    }
    Ok(())
}

fn main() {
    let mut args = std::env::args();

    args.next();

    let base_dir_path = args.next();

    match base_dir_path {
        Some(path) => {
            let base_dir = Path::new(&path);
            let mut size = Size::new();
            let _ = read_recursive(base_dir, &mut size);
        }
        None => {
            panic!("No base directory path provided.");
        }
    }
}
