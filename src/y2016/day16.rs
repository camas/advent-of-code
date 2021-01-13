pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let initial = input
        .trim()
        .chars()
        .map(|c| match c {
            '1' => true,
            '0' => false,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let part1 = disk_checksum(&initial, 272);
    let part2 = disk_checksum(&initial, 35651584);

    (part1, part2)
}

fn disk_checksum(initial: &[bool], length: usize) -> String {
    // Initialize an array with the maximum size needed
    let mut big_length = initial.len();
    while big_length < length {
        big_length = 2 * big_length + 1;
    }
    let mut data = vec![false; big_length];

    // Copy initial data
    data[..initial.len()].copy_from_slice(initial);

    // Loop until array filled
    let mut end = initial.len();
    while end < length {
        // Unecessary as already initialised to false
        // data[end] = false;
        // Copy a to b
        data.copy_within(..end, end + 1);
        // Reverse b
        data[(end + 1)..(end + 1 + end)].reverse();
        // Invert b
        data[(end + 1)..(end + 1 + end)]
            .iter_mut()
            .for_each(|b| *b = !*b);
        // Update end
        end = 2 * end + 1;
    }

    // Take length items from data
    let mut data = data.into_iter().take(length).collect::<Vec<_>>();
    // Loop while even length
    while data.len() % 2 == 0 {
        // Checksum
        data = data
            .chunks_exact(2)
            .map(|d| (d[0] && d[1]) || (!d[0] && !d[1]))
            .collect();
    }

    // Return as string
    data.into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect::<String>()
}
