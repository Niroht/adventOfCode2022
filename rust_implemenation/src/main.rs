use crate::days::day_one;

pub mod days;
pub mod inputs;

fn main() {
    let day_one_result = day_one::part_one(inputs::DAY_ONE);
    println!(
            "Part One: 1-Indexed Elf {index}, sum {sum}", 
            index=day_one_result.0 + 1,
            sum=day_one_result.1
        );

    println!("Part Two: Top Three Total {:?}", day_one::part_two(inputs::DAY_ONE));
}
