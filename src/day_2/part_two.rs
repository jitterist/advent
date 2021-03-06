pub fn get_common_chars_of_twin_boxes() -> String {
    let twins = get_twin_boxes();
    get_common_chars(&twins.0, &twins.1)
}

fn get_twin_boxes() -> (String, String) {
    let box_ids = super::utils::get_box_ids();
    let mut result = (String::new(), String::new());

    for i in 0..box_ids.len() - 2 {
        let first = &box_ids[i];
        for second in box_ids.iter().take(box_ids.len() - 1).skip(i + 1) {
            if are_equal_bar_one(first, second) {
                result = (first.clone(), second.clone())
            }
        }
    }

    result
}

fn are_equal_bar_one(first: &str, second: &str) -> bool {
    let mut counter = 0;
    first.chars().zip(second.chars()).for_each(|char_pair| {
        if char_pair.0 != char_pair.1 {
            counter += 1;
        }
    });

    counter == 1
}

fn get_common_chars(first: &str, second: &str) -> String {
    first
        .chars()
        .zip(second.chars())
        .filter(|char_pair| char_pair.0 == char_pair.1)
        .map(|char_pair| char_pair.0)
        .collect()
}
