use std::{collections::HashMap, str::FromStr};

use num::Integer;

use crate::common::Vector2;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let shapes = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"
    .split("\n\n")
    .map(|lines| lines.parse::<Shape>().unwrap())
    .collect::<Vec<_>>();

    let moves = input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let part1 = measure_tower(&shapes, &moves, 2022);
    let part2 = measure_tower(&shapes, &moves, 1_000_000_000_000);

    (part1, part2)
}

fn measure_tower(shapes: &[Shape], moves: &[Move], drop_count: usize) -> usize {
    let mut tower = Tower {
        layers: Vec::new(),
        moves: moves.to_vec(),
        move_index: 0,
    };

    let mut seen = HashMap::new();
    for i in 0..drop_count {
        let shape_index = i % shapes.len();
        let shape = &shapes[shape_index];
        tower.drop_shape(shape);

        let hash = tower.hash_top_layers();
        if let Some((first_i, first_height)) =
            seen.insert((shape_index, tower.move_index, hash), (i, tower.height()))
        {
            let cycle_size = i - first_i;
            let cycle_height = tower.height() - first_height;
            let (skipped_cycles, remaining_iterations) = (drop_count - 1 - i).div_rem(&cycle_size);
            let skipped_height = skipped_cycles * cycle_height;

            for j in (i + 1)..(i + 1 + remaining_iterations) {
                let shape_index = j % shapes.len();
                let shape = &shapes[shape_index];
                tower.drop_shape(shape);
            }
            return tower.layers.len() + skipped_height;
        }

        // println!();
        // for layer in tower.layers.iter().rev() {
        //     let layer_str = layer
        //         .iter()
        //         .map(|v| if *v { '#' } else { '.' })
        //         .collect::<String>();
        //     println!("{}", layer_str);
        // }
    }

    tower.height()
}

#[derive(Debug)]
struct Tower {
    layers: Vec<Vec<bool>>,
    moves: Vec<Move>,
    move_index: usize,
}

impl Tower {
    fn drop_shape(&mut self, shape: &Shape) {
        // Position of bottom left of shape
        let mut shape_pos = Vector2::new(2, self.height() as i64 + 3);

        loop {
            let after_gas_pos = match self.next_move() {
                Move::Left => shape_pos + Vector2::new(-1, 0),
                Move::Right => shape_pos + Vector2::new(1, 0),
            };
            if self.valid_shape_pos(shape, after_gas_pos) {
                shape_pos = after_gas_pos;
            }

            let after_drop_pos = shape_pos + Vector2::new(0, -1);
            if self.valid_shape_pos(shape, after_drop_pos) {
                shape_pos = after_drop_pos;
            } else {
                break;
            }
        }

        // Persist shape
        for (y, row) in shape.blocks.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                if !*block {
                    continue;
                }

                let layer_pos = Vector2::new(x as i64, y as i64) + shape_pos;
                self.persist_block(layer_pos);
            }
        }
    }

    fn persist_block(&mut self, position: Vector2) {
        while (self.layers.len() as i64) <= position.y {
            self.layers.push(vec![false; 7]);
        }
        self.layers[position.y as usize][position.x as usize] = true;
    }

    fn valid_shape_pos(&self, shape: &Shape, position: Vector2) -> bool {
        if position.x < 0 || position.x > (7 - shape.width() as i64) {
            return false;
        }

        if position.y < 0 {
            return false;
        }

        for (y, row) in shape.blocks.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                if !*block {
                    continue;
                }

                let layer_pos = Vector2::new(x as i64, y as i64) + position;
                if self.block_at(layer_pos) {
                    return false;
                }
            }
        }

        true
    }

    fn hash_top_layers(&self) -> u64 {
        let mut hash = 0;
        let mut i = 0;
        'outer: for layer in self.layers.iter().rev() {
            for v in layer {
                if *v {
                    hash |= 1 << i;
                }
                i += 1;
                if i >= u64::BITS {
                    break 'outer;
                }
            }
        }
        hash
    }

    fn block_at(&self, position: Vector2) -> bool {
        if position.y >= self.layers.len() as i64 {
            return false;
        }
        self.layers[position.y as usize][position.x as usize]
    }

    fn next_move(&mut self) -> Move {
        let m = self.moves[self.move_index];
        self.move_index = (self.move_index + 1) % self.moves.len();
        m
    }

    fn height(&self) -> usize {
        self.layers.len()
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
}

#[derive(Debug)]
struct Shape {
    blocks: Vec<Vec<bool>>,
}

impl Shape {
    fn width(&self) -> usize {
        self.blocks[0].len()
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .rev()
            .collect();

        Ok(Shape { blocks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 3068.to_string());
        assert_eq!(result.1.to_string(), 1514285714288_i64.to_string());
    }
}
