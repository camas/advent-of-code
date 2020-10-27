use crate::Exercise;

pub struct Day8;

impl Exercise for Day8 {
    fn part1(&self, input: &str) -> String {
        let mut mem_count = 0;
        let mut code_count = 0;
        for line in input.lines() {
            let data = &line.chars().collect::<Vec<_>>()[1..(line.len() - 1)];
            code_count += data.len() + 2;
            let mut decoded = Vec::new();
            let mut met_backslash = false;
            let mut met_hex = 0;
            let mut hex_tmp = Vec::new();
            for c in data {
                if met_backslash {
                    match c {
                        '\\' | '"' => {
                            decoded.push(*c);
                            met_backslash = false;
                        }
                        'x' => {
                            met_hex = 1;
                            met_backslash = false;
                        }
                        _ => unreachable!(),
                    }
                } else if met_hex > 0 {
                    hex_tmp.push(c);
                    met_hex += 1;
                    if met_hex > 2 {
                        met_hex = 0;
                        decoded.push(
                            u8::from_str_radix(&hex_tmp.iter().cloned().collect::<String>(), 16)
                                .unwrap() as char,
                        );
                        hex_tmp = Vec::new();
                    }
                } else if c == &'\\' {
                    met_backslash = true;
                } else {
                    decoded.push(*c);
                }
            }
            mem_count += decoded.len();
        }
        (code_count - mem_count).to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let line = line.chars().collect::<Vec<_>>();
                line.iter().filter(|&&c| c == '\\' || c == '"').count() + 2
            })
            .sum::<usize>()
            .to_string()
    }
}
