pub fn part_one(input: &str) -> (usize, i64) {
    part_one_impl(parse_input(input))
}

pub fn part_two(input: &str) -> i64 {
    part_two_impl(parse_input(input))
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let groups_of_newline_strings = input.split("\n\n");
    let groups_of_numbers = groups_of_newline_strings
        .into_iter()
        .map(|x| 
            x.split("\n")
            .into_iter()
            .map(|y| 
                y.parse::<i64>().unwrap()
            )
            .collect::<Vec<i64>>()
        );
    groups_of_numbers.collect::<Vec<Vec<i64>>>()
}

fn part_one_impl(input: Vec<Vec<i64>>) -> (usize, i64) {
    let mut highest_sum: i64 = 0;
    let mut highest_index: usize = 0;

    for i in 0..input.len()-1 {
        let sum = input[i].iter().sum();
        if sum > highest_sum {
            highest_sum = sum;
            highest_index = i;
        }
    }

    (highest_index, highest_sum)
}

fn part_two_impl(input: Vec<Vec<i64>>) -> i64 {
    let mut sums = input.iter().map(|x| x.iter().sum()).collect::<Vec<i64>>();
    sums.sort();
    sums.reverse();

    sums[0] + sums[1] + sums[2]
}