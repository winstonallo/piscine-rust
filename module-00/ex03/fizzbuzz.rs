fn fizzbuzz() {
    for i in 1..=100 {
        match i {
            _ if i % 3 == 0 && i % 5 == 0 => println!("fizzbuzz"),
            _ if i % 3 == 0 => println!("fizz"),
            _ if i % 5 == 0 => println!("buzz"),
            14 => println!("FIZZ"),
            16 => println!("BUZZ"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    fizzbuzz();
}