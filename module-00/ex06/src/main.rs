use ftkit::{read_number, random_number};
use std::cmp::Ordering;

fn main() {
    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");

    let secret_number = random_number(1..=100);

    loop {
        let guess: i32 = read_number();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            Ordering::Greater => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            Ordering::Equal => {
                println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", secret_number);
                break;
            }
        }
    }
}
