use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let (symbols, numbers) = parse_input(input);

    let part1 = numbers
        .iter()
        .filter(|number| symbols.iter().any(|symbol| are_adjacent(symbol, number)))
        .map(|number| number.value as u64)
        .sum::<u64>();

    let part2 = symbols
        .iter()
        .filter(|symbol| symbol.char == '*')
        .map(|symbol| {
            numbers
                .iter()
                .filter(|number| are_adjacent(symbol, number))
                .collect::<Vec<_>>()
        })
        .filter(|adjacent| adjacent.len() == 2)
        .map(|adjacent| adjacent[0].value as u64 * adjacent[1].value as u64)
        .sum::<u64>();

    (part1, part2)
}

struct Symbol {
    char: char,
    position: Vector2,
}

struct Number {
    value: u32,
    line: i64,
    column_start: i64,
    column_end: i64,
}

fn parse_input(input: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let y = y as i64;
        let mut current_number = None;
        let mut number_start = 0;
        for (x, char) in line.chars().enumerate() {
            let x = x as i64;
            match char.to_digit(10) {
                Some(digit) => {
                    if current_number.is_none() {
                        current_number = Some(digit);
                        number_start = x;
                    } else {
                        current_number = Some(current_number.unwrap() * 10 + digit);
                    }
                }
                None => {
                    if char.is_ascii_punctuation() && char != '.' {
                        symbols.push(Symbol {
                            char,
                            position: Vector2::new(x, y),
                        });
                    }
                    if let Some(current_number) = current_number.take() {
                        numbers.push(Number {
                            value: current_number,
                            line: y,
                            column_start: number_start,
                            column_end: x - 1,
                        });
                    }
                }
            }
        }

        if let Some(current_number) = current_number {
            numbers.push(Number {
                value: current_number,
                line: y,
                column_start: number_start,
                column_end: line.len() as i64 - 1,
            });
        }
    }

    (symbols, numbers)
}

fn are_adjacent(symbol: &Symbol, number: &Number) -> bool {
    ((number.column_start - 1)..=(number.column_end + 1)).contains(&symbol.position.x)
        && ((number.line - 1)..=(number.line + 1)).contains(&symbol.position.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "4361");
        assert_eq!(part2.to_string(), "467835");
    }
}
