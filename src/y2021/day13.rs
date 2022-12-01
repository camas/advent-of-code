use std::fmt::Display;

use crate::common::parse_letters;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    // Parse input
    let mut dots = Vec::new();
    let mut folds = Vec::new();
    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let (x, y) = line.split_once(',').unwrap();
        dots.push((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }
    for line in lines {
        let line = line.trim_start_matches("fold along ");
        let (dir, pos) = line.split_once('=').unwrap();
        let pos = pos.parse::<usize>().unwrap();
        folds.push(match dir {
            "x" => Fold::Vertical(pos),
            "y" => Fold::Horizontal(pos),
            _ => unreachable!(),
        });
    }

    let mut grid = Grid::from_dots(&dots);
    grid.do_fold(&folds[0]);
    let part1 = grid.dot_count();

    for fold in &folds[1..] {
        grid.do_fold(fold);
    }
    let part2 = parse_letters(&grid.dots);

    (part1, part2)
}

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug, Clone)]
struct Grid {
    dots: Vec<Vec<bool>>,
}

impl Grid {
    fn from_dots(dot_coords: &[(usize, usize)]) -> Self {
        let x_max = dot_coords.iter().map(|(x, _)| *x).max().unwrap();
        let y_max = dot_coords.iter().map(|(_, y)| *y).max().unwrap();
        let mut dots = vec![vec![false; x_max + 1]; y_max + 1];
        for (x, y) in dot_coords {
            dots[*y][*x] = true;
        }
        Self { dots }
    }

    fn dot_count(&self) -> usize {
        self.dots
            .iter()
            .map(|row| row.iter().filter(|&&d| d).count())
            .sum()
    }

    fn do_fold(&mut self, fold: &Fold) {
        match fold {
            Fold::Horizontal(pos) => self.fold_horizontal(*pos),
            Fold::Vertical(pos) => self.fold_vertical(*pos),
        }
    }

    /// Folds along a horizontal line
    fn fold_horizontal(&mut self, pos: usize) {
        assert_eq!(self.dots.len(), 2 * pos + 1);
        for i in 0..pos {
            let row = self.dots.pop().unwrap();
            for (j, v) in row.into_iter().enumerate() {
                self.dots[i][j] |= v;
            }
        }
        self.dots.pop().unwrap();
    }

    /// Folds along a vertical line
    fn fold_vertical(&mut self, pos: usize) {
        assert_eq!(self.dots[0].len(), 2 * pos + 1);
        for row in self.dots.iter_mut() {
            for i in 0..pos {
                let v = row.pop().unwrap();
                row[i] |= v;
            }
            row.pop().unwrap();
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.dots.iter() {
            for dot in row.iter() {
                write!(f, "{}", if *dot { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
