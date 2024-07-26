use ex07::lib::strpcmp;
use ftkit::ARGS;

fn main() {
    if ARGS.len() != 3 {
        panic!("invalid number of arguments");
    }
    match strpcmp(&ARGS[1].as_bytes(), &ARGS[2].as_bytes()) {
        true => println!("yes"),
        false => println!("no"),
    }
}
