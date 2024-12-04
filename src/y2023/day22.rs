use crate::common::Vector3;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut blocks = parse_input(input);
    blocks.sort_by_key(|block| block.position.z);

    for i in 0..blocks.len() {
        let block = &blocks[i];

        let falls_to_z = (0..i)
            .map(|j| &blocks[j])
            .find(|other| block.is_above(other))
            .map(|block| block.position.z + block.size.z + 1)
            .unwrap_or(1);

        blocks[i].position.z = falls_to_z;
    }

    blocks.sort_by_key(|block| block.position.z);
    dbg!(&blocks);

    let above_counts = blocks
        .iter()
        .enumerate()
        .map(|(i, block)| {
            blocks
                .iter()
                .enumerate()
                .filter(|(j, other)| i != *j && other.is_above(block))
                .count()
        })
        .collect::<Vec<_>>();
    dbg!(&above_counts);

    let part1 = above_counts.iter().filter(|count| **count == 1).count();

    (part1, "")
}

#[derive(Debug)]
struct Block {
    position: Vector3,
    size: Vector3,
}

impl Block {
    fn new(position_a: Vector3, position_b: Vector3) -> Block {
        let position = Vector3::new(
            position_a.x.min(position_b.x),
            position_a.y.min(position_b.y),
            position_a.z.min(position_b.z),
        );

        let size = Vector3::new(
            (position_a.x - position_b.x).abs(),
            (position_a.y - position_b.y).abs(),
            (position_a.z - position_b.z).abs(),
        );

        Block { position, size }
    }

    fn is_above(&self, other: &Block) -> bool {
        self.position.x <= other.position.x + other.size.x
            && self.position.x + self.size.x >= other.position.x
            && self.position.y <= other.position.y + other.size.y
            && self.position.y + self.size.y >= other.position.y
            && self.position.z > other.position.z + other.size.z
    }
}

fn parse_input(input: &str) -> Vec<Block> {
    input
        .lines()
        .map(|line| {
            let (position_a, position_b) = line.split_once('~').unwrap();
            Block::new(parse_vector3(position_a), parse_vector3(position_b))
        })
        .collect()
}

fn parse_vector3(block_str: &str) -> Vector3 {
    let mut parts = block_str.split(',').map(|v| v.parse::<i64>().unwrap());

    Vector3::new(
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        let (part1, _) = solve(input);

        assert_eq!(part1.to_string(), "5");
    }
}
