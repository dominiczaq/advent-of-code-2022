fn main() {
    let file_contents = std::fs::read_to_string("input.txt").expect("File contents should be read");
    let mut all_found_matches: Vec<char> = vec![];
    file_contents.lines().for_each(|line| {
        let (str1, str2) = line.split_at(line.len() / 2);
        let found_char = find_matched_char_in_both_strings_slices(str1, str2);
        if found_char.is_some() {
            all_found_matches.push(found_char.unwrap());
        }
    });
    let mut lookup: Vec<_> = ('a'..='z').collect();
    let lookup2: Vec<_> = ('A'..='Z').collect();
    lookup.extend(lookup2);

    let mut sum = 0;
    for m in &all_found_matches {
        let position = lookup.iter().position(|lookup_element| m == lookup_element);
        sum += position.unwrap() + 1;
    }

    println!("Sum: {}", sum);
}

fn find_matched_char_in_both_strings_slices(str1: &str, str2: &str) -> Option<char> {
    let vec1: Vec<char> = str1.chars().collect();
    let vec2: Vec<char> = str2.chars().collect();
    for char in vec1 {
        if vec2.contains(&char) {
            return Some(char);
        }
    }
    return None;
}
