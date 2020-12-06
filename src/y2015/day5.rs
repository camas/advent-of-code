pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let part1 = input
        .lines()
        .filter(|line| {
            let mut chars = line.chars();
            let mut last_char = chars.next().unwrap();
            const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
            let mut vowel_count = if VOWELS.contains(&last_char) { 1 } else { 0 };
            let mut contains_double = false;
            for c in chars {
                if VOWELS.contains(&c) {
                    vowel_count += 1;
                }
                if c == last_char {
                    contains_double = true;
                }
                match (last_char, c) {
                    ('a', 'b') => return false,
                    ('c', 'd') => return false,
                    ('p', 'q') => return false,
                    ('x', 'y') => return false,
                    _ => (),
                }
                last_char = c;
            }
            vowel_count >= 3 && contains_double
        })
        .count();

    let part2 = input
        .lines()
        .filter(|line| {
            let line = line.chars().collect::<Vec<_>>();
            let has_pair = (0..(line.len() - 2)).any(|i| {
                let to_check = &line[i..i + 2];
                ((i + 2)..(line.len() - 1)).any(|j| to_check == &line[j..(j + 2)])
            });
            let has_repeat = line.windows(3).any(|window| window[0] == window[2]);
            has_pair && has_repeat
        })
        .count();

    (part1, part2)
}
