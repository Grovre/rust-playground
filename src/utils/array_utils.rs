use std::fmt::{format, Display};

pub fn print_array<T>(array: &[T]) -> String
where
    T: Display,
{
    let mut str = String::new();
    let mut count = 0;
    for element in array {
        let el_str = format!("[{}] = {}\n", count, element);
        count += 1;
        str.push_str(el_str.as_str());
    }

    str
}
