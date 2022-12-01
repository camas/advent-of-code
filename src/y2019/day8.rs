use crate::common::parse_letters;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let digits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let width = 25;
    let height = 6;
    let layers = digits.chunks_exact(width * height).collect::<Vec<_>>();

    let smallest_index = layers
        .iter()
        .enumerate()
        .map(|(i, layer)| {
            let zeros = layer.iter().filter(|&&x| x == 0).count();
            (zeros, i)
        })
        .min()
        .unwrap()
        .1;
    let part1 = layers[smallest_index].iter().filter(|&&x| x == 1).count()
        * layers[smallest_index].iter().filter(|&&x| x == 2).count();

    let combined = (0..layers[0].len())
        .map(|i| {
            for layer in &layers {
                if layer[i] != 2 {
                    return layer[i];
                }
            }
            2
        })
        .collect::<Vec<_>>();
    // for (i, v) in combined.iter().enumerate() {
    //     if i % width == 0 && i != 0 {
    //         println!();
    //     }
    //     print!("{}", if *v == 1 { '#' } else { ' ' });
    // }
    let dots = combined
        .chunks(width)
        .map(|row| row.iter().map(|v| *v == 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let part2 = parse_letters(&dots);

    (part1, part2)
}
