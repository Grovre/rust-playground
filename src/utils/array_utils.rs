use std::fmt::Display;

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

pub fn max_iter<T>(array: &[T]) -> &T
where
    T: Ord,
{
    array.iter().max().unwrap()
}

pub fn max<T>(array: &[T]) -> &T
where
    T: Ord,
{
    let mut max = &array[0];
    for el in array {
        if max < el {
            max = el;
        }
    }

    max
}

pub fn max_with_keys<T>(array: &[T], key_extractor: fn(o: &T) -> usize) -> &T {
    let mut max = &array[0];
    let mut max_key = key_extractor(&array[0]);
    for el in array {
        let usz = key_extractor(el);
        if max_key < usz {
            max_key = usz;
            max = el;
        }
    }

    max
}

pub fn counting_sort<T>(_array: &[T], _key_extractor: fn(o: &T) -> usize)
where
    T: Ord,
{
}
