use std::fmt;
struct John;

impl fmt::Display for John {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.precision() {
            Some(0) => f.write_str("Don't try to silence me!"),
            _ => f.pad("Hey! I'm John.")
        }
    }
}

impl fmt::Debug for John {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("John, the man himself.")?;

        if f.alternate() {
            f.write_str(" He's handsome AND formidable.")?;
        }

        Ok(())
    }
}

impl fmt::Binary for John {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Bip Boop?")
    }
}

fn main() {
    let john = John;
    
    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}
