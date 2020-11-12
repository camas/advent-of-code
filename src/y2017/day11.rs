use crate::Exercise;

pub struct Day11;

impl Exercise for Day11 {
    fn part1(&self, input: &str) -> String {
        let moves = input.trim().split(',');

        let mut nw_count: i32 = 0;
        let mut n_count = 0;
        let mut ne_count = 0;
        let mut sw_count = 0;
        let mut s_count = 0;
        let mut se_count = 0;
        for m in moves {
            match m {
                "nw" => nw_count += 1,
                "n" => n_count += 1,
                "ne" => ne_count += 1,
                "sw" => sw_count += 1,
                "s" => s_count += 1,
                "se" => se_count += 1,
                _ => panic!(),
            }
        }
        let x = nw_count - se_count;
        let y = n_count - s_count;
        let z = ne_count - sw_count;
        let no_x = (y + x).abs() + (z - x).abs();
        let no_y = (x + y).abs() + (z + y).abs();
        let no_z = (x - z).abs() + (y + z).abs();
        no_x.min(no_y).min(no_z).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let moves = input.trim().split(',');

        let mut nw_count: i32 = 0;
        let mut n_count = 0;
        let mut ne_count = 0;
        let mut sw_count = 0;
        let mut s_count = 0;
        let mut se_count = 0;
        let mut best = i32::MIN;
        for m in moves {
            match m {
                "nw" => nw_count += 1,
                "n" => n_count += 1,
                "ne" => ne_count += 1,
                "sw" => sw_count += 1,
                "s" => s_count += 1,
                "se" => se_count += 1,
                _ => panic!(),
            }

            let x = nw_count - se_count;
            let y = n_count - s_count;
            let z = ne_count - sw_count;
            let no_x = (y + x).abs() + (z - x).abs();
            let no_y = (x + y).abs() + (z + y).abs();
            let no_z = (x - z).abs() + (y + z).abs();
            let steps = no_x.min(no_y).min(no_z);
            if steps > best {
                best = steps;
            }
        }
        best.to_string()
    }
}
