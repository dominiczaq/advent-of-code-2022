use std::fs;

fn main() {
    println!("Hello, world!");
    let file_contents = fs::read_to_string("input.txt").expect("File contents should be read");
    let split: Vec<&str> = file_contents
        .split("\n")
        // .filter(|&x| !x.is_empty())
        .collect();

    //part 1
    let mut max = 0;
    let mut local_sum = 0;
    let mut elves_sums: Vec<i32> = vec![];
    for val in split {
        let is_empty = val.is_empty();
        if is_empty {
            if local_sum > max {
                max = local_sum;
            }
            elves_sums.push(local_sum);
            local_sum = 0;
        } else {
            let val: i32 = val.parse().unwrap_or(0);
            local_sum += val;
        }
    }
    println!("Max: {:?}", max);

    //part 2
    elves_sums.sort();
    elves_sums.reverse();
    let last_3 = elves_sums.get(0..3);
    let sum_of_last_3: i32 = last_3.expect("Should have 3 top calories").iter().sum();

    println!("Sum of last 3: {:?}", sum_of_last_3);
}
