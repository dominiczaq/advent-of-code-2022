fn main() {
    let file_contents = std::fs::read_to_string("input.txt").expect("File contents should be read");
    let mut sum_of_intersected_ranges = 0;
    let mut sum_of_overlapping_ranges = 0;

    file_contents.lines().for_each(|line| {
        let sections = line.split(",");

        let mut first_range = None;
        sections.for_each(|section| {
            let mut range = section.split("-");
            let start: u32 = range.next().unwrap().parse().unwrap();
            let end: u32 = range.next().unwrap().parse().unwrap();
            let range = Range { start, end };
            match &first_range {
                Some(first_range) => {
                    if check_if_one_range_is_within_another(first_range, &range) {
                        sum_of_intersected_ranges += 1
                    }
                    if check_if_one_range_overlaps_with_another(first_range, &range) {
                        sum_of_overlapping_ranges += 1
                    }
                }
                None => first_range = Some(range),
            }
        })
    });
    println!("Sum of intersected ranges: {}", sum_of_intersected_ranges);
    println!("Sum of overlapping ranges: {}", sum_of_overlapping_ranges);
}

struct Range {
    start: u32,
    end: u32,
}

fn check_if_one_range_is_within_another(range1: &Range, range2: &Range) -> bool {
    if range1.start >= range2.start && range1.end <= range2.end {
        return true;
    }
    if range1.start <= range2.start && range1.end >= range2.end {
        return true;
    }
    false
}

#[test]
fn check_if_one_range_is_within_another_test() {
    let range1 = Range { start: 1, end: 5 };
    let range2 = Range { start: 0, end: 10 };
    assert_eq!(check_if_one_range_is_within_another(&range1, &range2), true);
    let range1 = Range { start: 1, end: 5 };
    let range2 = Range { start: 0, end: 4 };
    assert_eq!(
        check_if_one_range_is_within_another(&range1, &range2),
        false
    );
    let range1 = Range { start: 0, end: 10 };
    let range2 = Range { start: 1, end: 5 };
    assert_eq!(check_if_one_range_is_within_another(&range1, &range2), true);
}

#[test]
fn check_if_one_range_overlaps_with_another_test() {
    let range1 = Range { start: 1, end: 5 };
    let range2 = Range { start: 0, end: 10 };
    assert_eq!(
        check_if_one_range_overlaps_with_another(&range1, &range2),
        true
    );
    let range1 = Range { start: 1, end: 5 };
    let range2 = Range { start: 0, end: 4 };
    assert_eq!(
        check_if_one_range_overlaps_with_another(&range1, &range2),
        true
    );
    let range1 = Range { start: 0, end: 10 };
    let range2 = Range { start: 1, end: 5 };
    assert_eq!(
        check_if_one_range_overlaps_with_another(&range1, &range2),
        true
    );
    let range1 = Range { start: 0, end: 10 };
    let range2 = Range { start: 11, end: 20 };
    assert_eq!(
        check_if_one_range_overlaps_with_another(&range1, &range2),
        false
    );
    let range1 = Range { start: 0, end: 10 };
    let range2 = Range { start: 10, end: 15 };
    assert_eq!(
        check_if_one_range_overlaps_with_another(&range1, &range2),
        true
    );
    let range1 = Range { start: 10, end: 20 };
    let range2 = Range { start: 0, end: 10 };
    assert_eq!(
        check_if_one_range_overlaps_with_another(&range1, &range2),
        true
    );
}

fn check_if_one_range_overlaps_with_another(range1: &Range, range2: &Range) -> bool {
    if range1.start >= range2.start && range1.start <= range2.end {
        return true;
    }
    if range1.end >= range2.start && range1.end <= range2.end {
        return true;
    }
    if range2.start >= range1.start && range2.start <= range1.end {
        return true;
    }
    if range2.end >= range1.start && range2.end <= range1.end {
        return true;
    }
    false
}
