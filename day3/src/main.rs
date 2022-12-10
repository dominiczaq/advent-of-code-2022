fn main() {
    let file_contents = std::fs::read_to_string("input.txt").expect("File contents should be read");
    let mut all_found_matches: Vec<char> = vec![];
    file_contents.lines().for_each(|line| {
        let (str1, str2) = line.split_at(line.len() / 2);
        let found_char = find_matched_char_in_array_of_strings_slices(&[&str1, &str2]);
        if found_char.is_some() {
            all_found_matches.push(found_char.unwrap());
        }
    });
    let lookup = make_alphabet_lookup();

    let mut sum = 0;
    for m in &all_found_matches {
        let position = get_position_value_in_alphabet(m, &lookup);
        sum += position.unwrap() + 1;
    }

    println!("Sum: {}", sum);

    // part 2
    // split file_contents into lines of 3
    let mut lines: Vec<&str> = file_contents.lines().collect();
    let mut sum = 0;
    while lines.len() > 0 {
        let mut line_of_3: Vec<&str> = vec![];
        for _ in 0..3 {
            line_of_3.push(lines.remove(0));
        }
        let matched_char = find_matched_char_in_array_of_strings_slices(line_of_3.as_slice());
        match matched_char {
            Some(c) => sum += get_position_value_in_alphabet(&c, &lookup).unwrap() + 1,
            None => panic!("No match found"),
        }
    }
    println!("Sum of matched in triplets: {}", sum);
}

fn get_position_value_in_alphabet(m: &char, lookup: &[char]) -> Option<u32> {
    let position = lookup.iter().position(|lookup_element| m == lookup_element);
    match position {
        Some(p) => Some(p as u32),
        None => None,
    }
}

#[test]
fn get_position_value_in_alphabet_test() {
    let lookup = make_alphabet_lookup();
    let position = get_position_value_in_alphabet(&'a', &lookup);
    assert_eq!(position, Some(0));
    let position = get_position_value_in_alphabet(&'z', &lookup);
    assert_eq!(position, Some(25));
    let position = get_position_value_in_alphabet(&'A', &lookup);
    assert_eq!(position, Some(26));
    let position = get_position_value_in_alphabet(&'Z', &lookup);
    assert_eq!(position, Some(51));
}

fn find_matched_char_in_array_of_strings_slices(strings: &[&str]) -> Option<char> {
    let vec1: Vec<char> = strings[0].chars().collect();
    let size_of_strings = strings.len();
    for char in vec1 {
        let mut possible_match: Option<char> = None;
        for i in 1..size_of_strings {
            if strings[i].contains(char) {
                possible_match = Some(char);
                continue;
            } else {
                possible_match = None;
                break;
            }
        }
        if possible_match.is_some() {
            return possible_match;
        }
    }
    None
}

fn make_alphabet_lookup() -> Vec<char> {
    let mut lookup: Vec<_> = ('a'..='z').collect();
    let lookup2: Vec<_> = ('A'..='Z').collect();
    lookup.extend(lookup2);
    lookup
}

#[test]
fn find_matched_char_in_array_of_strings_slices_test() {
    let strings = ["abc", "def", "ghi"];
    let result = find_matched_char_in_array_of_strings_slices(&strings);
    assert_eq!(result, None);

    let strings = ["abc", "defa", "gahi"]; // should return 'a'
    let result = find_matched_char_in_array_of_strings_slices(&strings);
    assert_eq!(result, Some('a'));
}
