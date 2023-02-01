use std::fmt::Display;

fn main() {

}

fn print_array<T>(array: &[T])
where
    T: Display,
{
    for o in array {
        println!("{}", o);
    }
}
