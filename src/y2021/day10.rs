pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let lines = input.lines().collect::<Vec<_>>();

    let mut part1 = 0;
    let mut scores = Vec::new();
    for line in lines.iter() {
        let mut chunks = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => chunks.push(c),
                ')' => match chunks.pop() {
                    Some('(') => (),
                    _ => {
                        part1 += 3;
                        corrupted = true;
                        break;
                    }
                },
                ']' => match chunks.pop() {
                    Some('[') => (),
                    _ => {
                        part1 += 57;
                        corrupted = true;
                        break;
                    }
                },
                '}' => match chunks.pop() {
                    Some('{') => (),
                    _ => {
                        part1 += 1197;
                        corrupted = true;
                        break;
                    }
                },
                '>' => match chunks.pop() {
                    Some('<') => (),
                    _ => {
                        part1 += 25137;
                        corrupted = true;
                        break;
                    }
                },
                _ => unreachable!(),
            }
        }

        if !corrupted {
            let mut score = 0_i64;
            for c in chunks.iter().rev() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
            }
            scores.push(score);
        }
    }
    scores.sort_unstable();
    let part2 = scores[scores.len() / 2];

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let data = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let (part1, part2) = solve(data);
        assert_eq!(part1.to_string(), "26397");
        assert_eq!(part2.to_string(), "288957");
    }
}
