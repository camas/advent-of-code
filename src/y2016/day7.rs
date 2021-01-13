use std::collections::HashSet;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let ips = input.lines().collect::<Vec<_>>();

    let part1 = ips
        .iter()
        .filter(|ip| {
            let mut in_hypernet = false;
            let mut found = false;
            let chars = ip.chars().collect::<Vec<_>>();
            for c in chars.windows(4) {
                if c[0] == '[' {
                    in_hypernet = true;
                    continue;
                }
                if c[0] == ']' {
                    in_hypernet = false;
                    continue;
                }
                if c[0] == c[3] && c[1] == c[2] && c[0] != c[1] {
                    if in_hypernet {
                        return false;
                    } else {
                        found = true;
                    }
                }
            }
            found
        })
        .count();

    let part2 = ips
        .iter()
        .filter(|ip| {
            let mut in_hypernet = false;
            let mut supernet = HashSet::new();
            let mut hypernet = HashSet::new();
            let chars = ip.chars().collect::<Vec<_>>();
            for c in chars.windows(3) {
                if c[0] == '[' {
                    in_hypernet = true;
                    continue;
                }
                if c[0] == ']' {
                    in_hypernet = false;
                    continue;
                }
                if c[0] == c[2] && c[0] != c[1] {
                    if in_hypernet {
                        hypernet.insert((c[0], c[1]));
                    } else {
                        supernet.insert((c[0], c[1]));
                    }
                }
            }
            supernet
                .iter()
                .any(|a| hypernet.iter().any(|b| a.0 == b.1 && a.1 == b.0))
        })
        .count();

    (part1, part2)
}
