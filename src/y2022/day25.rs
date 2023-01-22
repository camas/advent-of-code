pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let part1 = to_snafu(input.trim().lines().map(from_snafu).sum());

    (part1, "")
}

fn from_snafu(line: &str) -> i64 {
    let mut exponent = 1;
    let mut n = 0;
    for c in line.chars().rev() {
        n += exponent
            * match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            };
        exponent *= 5;
    }
    n
}

fn to_snafu(mut n: i64) -> String {
    let mut nums = Vec::new();
    while n > 0 {
        nums.push(n % 5);
        n /= 5;
    }

    for i in 0.. {
        if i >= nums.len() {
            break;
        }
        while nums[i] > 2 {
            nums[i] -= 5;
            if i >= nums.len() - 1 {
                nums.push(0);
            }
            nums[i + 1] += 1;
        }
    }

    nums.into_iter()
        .map(|n| match n {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
        .rev()
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        let result = solve(input);

        assert_eq!(result.0.to_string(), "2=-1=0")
    }
}
