use rand::random;
use utils::array_utils::array_to_string;

use crate::utils::array_utils;

mod utils;

const LEN: usize = 5_000;

fn main() {
    let mut arr = [0i32; LEN];
    for i in 0..LEN {
        arr[i] = (random::<f32>() * 10f32) as i32;
    }

    let str = array_to_string(&arr);
    println!("{}", str);
}
