mod lib;

use lib::image;

fn main() {
    let data: &str = include_str!("input.txt");
    println!("Solution for part 1! {}", image::solve_part_1(data, 6, 25));
    image::solve_part_2(data, 6, 25);
}
