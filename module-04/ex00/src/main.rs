use std::fmt::Debug;

fn print_all_things<I>(i: I)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Debug,
{
    print!("[ ");
    for thing in i {
        print!("{:?} ", thing);
    }
    println!("]");
}

fn main() {
    print_all_things(0..=5);
    print_all_things("Hello".chars());
    print_all_things(vec![1, 3, 4, 2]);
    print_all_things([1, 2, 5, 4]);
}
