pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut indexes = input
        .trim()
        .chars()
        .map(|c| c as u8 - b'a')
        .collect::<Vec<_>>();

    'main: loop {
        // Increment
        for value in indexes.iter_mut().rev() {
            *value += 1;
            if *value >= 26 {
                *value = 0;
            } else {
                break;
            }
        }
        // Check
        let mut pairs = Vec::new();
        let mut found_pair_last_iter = false;
        let mut straight = false;
        let mut straight_len = 0;
        let mut last_index = 100; // Can't be 0-25 initially so as not to match
        for index in indexes.iter() {
            if *index == (b'i' - b'a') || *index == (b'o' - b'a') || *index == (b'l' - b'a') {
                continue 'main;
            }
            if *index == last_index && pairs.len() < 2 {
                if !found_pair_last_iter && !pairs.contains(index) {
                    found_pair_last_iter = true;
                    pairs.push(*index);
                } else {
                    found_pair_last_iter = false;
                }
            } else {
                found_pair_last_iter = false;
            }

            if *index == last_index + 1 {
                straight_len += 1;
                if straight_len >= 2 {
                    straight = true;
                }
            } else {
                straight_len = 0;
            }

            last_index = *index;
        }

        if pairs.len() == 2 && straight {
            break;
        }
    }

    let part1 = indexes
        .iter()
        .map(|i| (i + b'a') as char)
        .collect::<String>();

    let mut indexes = input
        .trim()
        .chars()
        .map(|c| c as u8 - b'a')
        .collect::<Vec<_>>();

    let mut found = 0;

    'main2: loop {
        // Increment
        for value in indexes.iter_mut().rev() {
            *value += 1;
            if *value >= 26 {
                *value = 0;
            } else {
                break;
            }
        }
        // Check
        let mut pairs = Vec::new();
        let mut found_pair_last_iter = false;
        let mut straight = false;
        let mut straight_len = 0;
        let mut last_index = 100; // Can't be 0-25 initially so as not to match
        for index in indexes.iter() {
            if *index == (b'i' - b'a') || *index == (b'o' - b'a') || *index == (b'l' - b'a') {
                continue 'main2;
            }
            if *index == last_index && pairs.len() < 2 {
                if !found_pair_last_iter && !pairs.contains(index) {
                    found_pair_last_iter = true;
                    pairs.push(*index);
                } else {
                    found_pair_last_iter = false;
                }
            } else {
                found_pair_last_iter = false;
            }

            if *index == last_index + 1 {
                straight_len += 1;
                if straight_len >= 2 {
                    straight = true;
                }
            } else {
                straight_len = 0;
            }

            last_index = *index;
        }

        if pairs.len() == 2 && straight {
            found += 1;
            if found == 2 {
                break;
            }
        }
    }

    let part2 = indexes
        .iter()
        .map(|i| (i + b'a') as char)
        .collect::<String>();

    (part1, part2)
}
