pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let wordsearch = input
        .trim()
        .split('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let part1 = search(&wordsearch);
    let part2 = xsearch(&wordsearch);

    (part1, part2)
}

fn search(wordsearch: &[&[u8]]) -> usize {
    const XMAS: &[u8] = b"XMAS";
    let rev_word = XMAS.iter().rev().cloned().collect::<Vec<_>>();

    let width = wordsearch[0].len();
    let height = wordsearch.len();
    let width_height_min = width.min(height);

    let mut count = 0;

    let count_words = |data: &[u8]| {
        data.windows(XMAS.len())
            .filter(|window| window == &XMAS || &rev_word == window)
            .count()
    };

    // Rows
    count += wordsearch
        .iter()
        .map(|line| count_words(line))
        .sum::<usize>();

    // Columns
    count += (0..wordsearch[0].len())
        .map(|x| {
            count_words(
                &(0..wordsearch.len())
                    .map(|y| wordsearch[y][x])
                    .collect::<Vec<_>>(),
            )
        })
        .sum::<usize>();

    // Diagonals - right/down, from top
    count += (0..=(width - XMAS.len()))
        .map(|start_x| {
            let end_x = (start_x + width_height_min).min(width - 1);
            let diagonal = (start_x..=end_x)
                .enumerate()
                .map(|(y, x)| wordsearch[y][x])
                .collect::<Vec<_>>();
            count_words(&diagonal)
        })
        .sum::<usize>();

    // Diagonals - right/down, from left
    count += (1..=(height - XMAS.len()))
        .map(|start_y| {
            let end_y = (start_y + width_height_min).min(height - 1);
            let diagonal = (start_y..=end_y)
                .enumerate()
                .map(|(x, y)| wordsearch[y][x])
                .collect::<Vec<_>>();
            count_words(&diagonal)
        })
        .sum::<usize>();

    // Diagonals - right/up, from bottom
    count += (0..=(width - XMAS.len()))
        .map(|start_x| {
            let end_x = (start_x + width_height_min).min(width - 1);
            let diagonal = (start_x..=end_x)
                .enumerate()
                .map(|(y, x)| wordsearch[height - y - 1][x])
                .collect::<Vec<_>>();
            count_words(&diagonal)
        })
        .sum::<usize>();

    // Diagonals - right/up, from left
    count += ((XMAS.len() - 1)..(height - 1))
        .map(|start_y| {
            let size = width_height_min.min(start_y);
            let end_y = start_y - size;
            let diagonal = (end_y..=start_y)
                .rev()
                .enumerate()
                .map(|(x, y)| wordsearch[y][x])
                .collect::<Vec<_>>();
            count_words(&diagonal)
        })
        .sum::<usize>();

    count
}

fn xsearch(wordsearch: &[&[u8]]) -> usize {
    const MAS: &[u8] = b"MAS";
    let width = wordsearch[0].len();
    let height = wordsearch.len();

    (0..=(width - MAS.len()))
        .map(|start_x| {
            (0..=(height - MAS.len()))
                .filter(|&start_y| {
                    wordsearch[start_y + 1][start_x + 1] == b'A'
                        && ((wordsearch[start_y][start_x] == b'M'
                            && wordsearch[start_y + 2][start_x + 2] == b'S')
                            || (wordsearch[start_y][start_x] == b'S'
                                && wordsearch[start_y + 2][start_x + 2] == b'M'))
                        && ((wordsearch[start_y + 2][start_x] == b'M'
                            && wordsearch[start_y][start_x + 2] == b'S')
                            || (wordsearch[start_y + 2][start_x] == b'S'
                                && wordsearch[start_y][start_x + 2] == b'M'))
                })
                .count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "18");
        assert_eq!(part2.to_string(), "9");
    }
}
