pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let data = input.trim();

    // Basic length calc
    let mut count = 0_u64;
    let mut chars = data.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let length = chars
                    .by_ref()
                    .take_while(|&c| c != 'x')
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let amount = chars
                    .by_ref()
                    .take_while(|&c| c != ')')
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                count += length * amount;
                // `advance_by` not stable yet
                let _ = chars.by_ref().take(length as usize).collect::<Vec<_>>();
            }
            _ => count += 1,
        }
    }
    let part1 = count;

    // Recursive length calc
    let chars = data.chars().collect::<Vec<_>>();
    let part2 = decompressed_length(&chars);

    (part1, part2)
}

fn decompressed_length(chars: &[char]) -> u64 {
    let mut count = 0_u64;
    let mut chars = chars.iter();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let length = chars
                    .by_ref()
                    .take_while(|&&c| c != 'x')
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let amount = chars
                    .by_ref()
                    .take_while(|&&c| c != ')')
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let inner = chars
                    .by_ref()
                    .take(length as usize)
                    .cloned()
                    .collect::<Vec<_>>();
                count += amount * decompressed_length(&inner);
            }
            _ => count += 1,
        }
    }
    count
}
