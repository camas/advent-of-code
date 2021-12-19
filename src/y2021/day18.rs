pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let snailfishes = input.lines().map(Segment::from_str).collect::<Vec<_>>();

    let mut num = snailfishes[0].clone();
    for other in &snailfishes[1..] {
        num = num.add(other.clone());
    }
    let part1 = num.magnitude();

    let mut best = i64::MIN;
    for (i, a) in snailfishes.iter().enumerate() {
        for (j, b) in snailfishes.iter().enumerate() {
            if i == j {
                continue;
            }
            let mag = a.clone().add(b.clone()).magnitude();
            if mag > best {
                best = mag;
            }
        }
    }
    let part2 = best;

    (part1, part2)
}

#[derive(Debug, Clone)]
enum Segment {
    Number(i64),
    Pair(Box<Segment>, Box<Segment>),
}

impl Segment {
    fn from_str(data: &str) -> Self {
        if data.starts_with('[') {
            assert!(data.ends_with(']'));
            let inner = &data[1..(data.len() - 1)];
            let mut split_index = None;
            let mut depth = 0;
            for (i, c) in inner.chars().enumerate() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => {
                        if depth == 0 {
                            split_index = Some(i);
                            break;
                        }
                    }
                    _ => (),
                }
            }
            let split_index = split_index.unwrap();
            let (left, right) = inner.split_at(split_index);
            let left = Segment::from_str(left);
            let right = Segment::from_str(&right[1..]);
            Segment::Pair(Box::new(left), Box::new(right))
        } else {
            Segment::Number(data.parse().unwrap())
        }
    }

    fn add(self, other: Segment) -> Self {
        let mut result = Segment::Pair(Box::new(self), Box::new(other));
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<i64>, Option<i64>)> {
        match self {
            Segment::Pair(left, right) => {
                if depth == 4 {
                    match (left.as_ref(), right.as_ref()) {
                        (Segment::Number(l), Segment::Number(r)) => {
                            let (l, r) = (*l, *r);
                            *self = Segment::Number(0);
                            Some((Some(l), Some(r)))
                        }
                        _ => None,
                    }
                } else if let Some((left_val, right_val)) = left.explode(depth + 1) {
                    if let Some(right_val) = right_val {
                        right.add_right(right_val);
                    }
                    Some((left_val, None))
                } else if let Some((left_val, right_val)) = right.explode(depth + 1) {
                    if let Some(left_val) = left_val {
                        left.add_left(left_val);
                    }
                    Some((None, right_val))
                } else {
                    None
                }
            }
            Segment::Number(_) => None,
        }
    }

    /// Add a number from the right of an explosion
    fn add_right(&mut self, value: i64) {
        match self {
            Segment::Pair(left, _) => left.add_right(value),
            Segment::Number(n) => {
                *n += value;
            }
        }
    }

    fn add_left(&mut self, value: i64) {
        match self {
            Segment::Pair(_, right) => right.add_left(value),
            Segment::Number(n) => {
                *n += value;
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Segment::Pair(left, right) => left.split() || right.split(),
            Segment::Number(n) => {
                if *n >= 10 {
                    *self = Segment::Pair(
                        Box::new(Segment::Number(*n / 2)),
                        Box::new(Segment::Number((*n / 2) + (*n % 2))),
                    );
                    true
                } else {
                    false
                }
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Segment::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
            Segment::Number(n) => *n,
        }
    }
}
