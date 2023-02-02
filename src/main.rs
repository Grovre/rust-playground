pub(crate) mod geometry2d;

use crate::geometry2d::point::Point;
use rand::random;
use std::fmt::Display;

fn main() {
    let mut vec_p = vec![];
    for _i in 0..3 {
        let x = random::<i32>() as f32;
        let y = random::<i32>() as f32;
        vec_p.push(Point::new(x, y));
    }

    print_array(&vec_p);
    let mut vec_l = Point::connect_points(&vec_p);
    let p1 = Point::new(0f32, 0f32).as_line(&Point::new(1f32, 1f32));
    let p2 = Point::new(1f32, 1f32).as_line(&Point::new(2f32, 2f32));
    println!("{} and {} are parallel: {}", &p1, &p2, p1.is_parallel(&p2));
    vec_l.push(p1);
    vec_l.push(p2);
    print_array(&vec_l);


}

fn print_array<T>(array: &[T])
where
    T: Display,
{
    for o in array {
        println!("{}", o);
    }
}
