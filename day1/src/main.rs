use std::fs;

fn main() {
    println!("Hello, world!");
    let file_contents = fs::read_to_string("input.txt").expect("File contents should be read");
    let split: Vec<&str> = file_contents
        .split("\n")
        // .filter(|&x| !x.is_empty())
        .collect();

    let mut max = 0;

    let mut local_sum = 0;
    for val in split {
        let is_empty = val.is_empty();
        if is_empty {
            if local_sum > max {
                max = local_sum;
            }
            local_sum = 0;
        } else {
            let val: i32 = val.parse().unwrap_or(0);
            local_sum += val;
        }
    }

    println!("{:?}", max);

    // println!("{:?}", split)
}
