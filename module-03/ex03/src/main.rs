use std::fmt::Debug;

trait FortyTwo {
    fn forty_two() -> Self;
}

impl FortyTwo for i32 {
    fn forty_two() -> Self {
        42
    }
}

impl FortyTwo for String {
    fn forty_two() -> Self {
        String::from("42")
    }
}

fn print_forty_two<T: Debug + FortyTwo>() {
    let ft = T::forty_two();
    println!("{ft:?}")
}

fn main() {
    print_forty_two::<i32>();
    print_forty_two::<String>();
}
