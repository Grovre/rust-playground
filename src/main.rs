mod utils;

const LEN: usize = 50;

fn main() {
    let arr = [0; LEN];
    let str = utils::array_utils::print_array(&arr);
    println!("{}", str);
}
