use std::{fmt::{Display}, thread::available_parallelism, borrow::Borrow};

pub fn array_to_string<T>(array: &[T]) -> String
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

pub fn max_iter<T>(array: &[T]) -> &T where T: Ord {
    array.iter().max().unwrap()
}

pub fn max<T>(array: &[T]) -> &T where T: Ord {
    let mut max = &array[0];
    for el in array {
        if max < el {
            max = el;
        }
    }

    max
}