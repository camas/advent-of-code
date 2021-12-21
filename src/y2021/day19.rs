use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use itertools::Itertools;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let scanners = input
        .split("\n\n")
        .map(Scanner::from_str)
        .collect::<Vec<_>>();

    // Find pairs of scanners that have overlapping beacons
    let mut matches = HashMap::<(usize, usize), OverlapMatch>::new();
    for (i, scanner) in scanners.iter().enumerate() {
        // Should work but doesn't
        // for (j, other) in scanners.iter().enumerate().skip(i + 1) {
        for (j, other) in scanners.iter().enumerate() {
            if i == j {
                continue;
            }
            let overlaps = scanner.find_overlaps(other);
            if overlaps.len >= 12 {
                matches.insert((j, i), overlaps.inverted());
                matches.insert((i, j), overlaps);
            }
        }
    }

    // Construct map
    let mut mapped_scanners = vec![ScannerPosition {
        position: Vector3 { x: 0, y: 0, z: 0 },
        orientation: Orientation::default(),
        scanner: 0,
    }];
    let mut queue = vec![0];
    while !queue.is_empty() {
        let scanner_index = queue.pop().unwrap();
        let m = matches
            .keys()
            .filter(|(a, b)| {
                *a == scanner_index && !mapped_scanners.iter().any(|m| m.scanner == *b)
            })
            .collect::<Vec<_>>();
        for (_, new_index) in m {
            let new_scanner = &scanners[*new_index];
            let (curr_pos, overlap) = mapped_scanners
                .iter()
                .find_map(|p| {
                    let overlap = matches.get(&(p.scanner, *new_index));
                    overlap.map(|o| (p, o))
                })
                .unwrap();

            // Find relative position from current scanner to new scanner
            let curr_scanner = &scanners[curr_pos.scanner];
            let offset = &curr_scanner.beacons[overlap.beacon_index]
                - &new_scanner.beacons[overlap.other_beacon_index].transform(&overlap.orientation);

            // Convert to map relative position
            let position = &curr_pos.position + &offset.transform(&curr_pos.orientation);
            let orientation = &curr_pos.orientation + &overlap.orientation;

            // Add to map
            mapped_scanners.push(ScannerPosition {
                position,
                orientation,
                scanner: *new_index,
            });
            queue.push(*new_index);
        }
    }
    let mut beacons = HashSet::new();
    for mapping in mapped_scanners.iter() {
        let scanner = &scanners[mapping.scanner];
        for beacon in scanner.beacons.iter() {
            beacons.insert(&mapping.position + &beacon.transform(&mapping.orientation));
        }
    }
    let part1 = beacons.len();

    let mut best = 0;
    for a in mapped_scanners.iter() {
        for b in mapped_scanners.iter() {
            let distance = a.position.distance(&b.position);
            if distance > best {
                best = distance;
            }
        }
    }
    let part2 = best;

    (part1, part2)
}

#[derive(Debug)]
struct ScannerPosition {
    position: Vector3,
    orientation: Orientation,
    scanner: usize,
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Vector3>,
    precalculated: Vec<Vec<Vec<Vector3>>>,
}

impl Scanner {
    fn from_str(data: &str) -> Self {
        let beacons = data
            .lines()
            .skip(1)
            .map(Vector3::from_str)
            .collect::<Vec<_>>();
        let precalculated = Orientation::all()
            .iter()
            .map(|o| {
                beacons
                    .iter()
                    .map(|b| {
                        beacons
                            .iter()
                            .map(|b2| (b2 - b).transform(o))
                            .sorted()
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            beacons,
            precalculated,
        }
    }

    fn find_overlaps(&self, other: &Scanner) -> OverlapMatch {
        let mut best_len = 0;
        let mut best_match = None;
        for orientation_index in 0..Orientation::all().len() {
            for i in 0..self.beacons.len() {
                let beacons = &self.precalculated[0][i];
                for j in 0..other.beacons.len() {
                    let other_beacons = &other.precalculated[orientation_index][j];
                    let overlaps = beacons
                        .iter()
                        .filter(|b| other_beacons.contains(b))
                        // .filter(|b| other_beacons.binary_search(b).is_ok())
                        .count();
                    if overlaps > best_len {
                        best_len = overlaps;
                        best_match = Some(OverlapMatch {
                            beacon_index: i,
                            other_beacon_index: j,
                            orientation: Orientation::all()[orientation_index],
                            len: overlaps,
                        });
                    }
                }
            }
        }
        best_match.unwrap()
    }
}

#[derive(Debug)]
struct OverlapMatch {
    beacon_index: usize,
    other_beacon_index: usize,
    orientation: Orientation,
    len: usize,
}

impl OverlapMatch {
    fn inverted(&self) -> Self {
        Self {
            beacon_index: self.other_beacon_index,
            other_beacon_index: self.beacon_index,
            orientation: self.orientation.inverse(),
            len: self.len,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn from_str(data: &str) -> Self {
        let parts = data
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }

    fn transform(&self, orientation: &Orientation) -> Self {
        Self {
            x: self.get_dimension(orientation.x),
            y: self.get_dimension(orientation.y),
            z: self.get_dimension(orientation.z),
        }
    }

    fn get_dimension(&self, transformation: Transformation) -> i64 {
        match transformation {
            Transformation::XNeg => -self.x,
            Transformation::XPos => self.x,
            Transformation::YNeg => -self.y,
            Transformation::YPos => self.y,
            Transformation::ZNeg => -self.z,
            Transformation::ZPos => self.z,
        }
    }

    fn distance(&self, other: &Vector3) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Orientation {
    x: Transformation,
    y: Transformation,
    z: Transformation,
}

impl Orientation {
    const fn all() -> &'static [Self; 24] {
        macro_rules! orientation {
            ($x:ident, $y:ident, $z:ident) => {{
                Orientation {
                    x: Transformation::$x,
                    y: Transformation::$y,
                    z: Transformation::$z,
                }
            }};
        }

        // This is all possible permutations of x/-x, y/-y, z/-z that can be obtained
        // by rotation
        //
        // XPos, YPos, ZNeg not possible for example because it is a reflection
        &[
            orientation!(XPos, YPos, ZPos),
            orientation!(XPos, YNeg, ZNeg),
            orientation!(XNeg, YPos, ZNeg),
            orientation!(XNeg, YNeg, ZPos),
            orientation!(XPos, ZPos, YNeg),
            orientation!(XPos, ZNeg, YPos),
            orientation!(XNeg, ZPos, YPos),
            orientation!(XNeg, ZNeg, YNeg),
            orientation!(YPos, XPos, ZNeg),
            orientation!(YPos, XNeg, ZPos),
            orientation!(YNeg, XPos, ZPos),
            orientation!(YNeg, XNeg, ZNeg),
            orientation!(ZPos, XPos, YNeg),
            orientation!(ZPos, XNeg, YPos),
            orientation!(ZNeg, XPos, YPos),
            orientation!(ZNeg, XNeg, YNeg),
            orientation!(YPos, ZPos, XPos),
            orientation!(YPos, ZNeg, XNeg),
            orientation!(YNeg, ZPos, XNeg),
            orientation!(YNeg, ZNeg, XPos),
            orientation!(ZPos, YPos, XNeg),
            orientation!(ZPos, YNeg, XPos),
            orientation!(ZNeg, YPos, XPos),
            orientation!(ZNeg, YNeg, XNeg),
        ]
    }

    fn inverse(&self) -> Self {
        let x = match (self.x, self.y, self.z) {
            (Transformation::XPos, _, _) => Transformation::XPos,
            (Transformation::XNeg, _, _) => Transformation::XNeg,
            (_, Transformation::XPos, _) => Transformation::YPos,
            (_, Transformation::XNeg, _) => Transformation::YNeg,
            (_, _, Transformation::XPos) => Transformation::ZPos,
            (_, _, Transformation::XNeg) => Transformation::ZNeg,
            _ => unreachable!(),
        };
        let y = match (self.x, self.y, self.z) {
            (Transformation::YPos, _, _) => Transformation::XPos,
            (Transformation::YNeg, _, _) => Transformation::XNeg,
            (_, Transformation::YPos, _) => Transformation::YPos,
            (_, Transformation::YNeg, _) => Transformation::YNeg,
            (_, _, Transformation::YPos) => Transformation::ZPos,
            (_, _, Transformation::YNeg) => Transformation::ZNeg,
            _ => unreachable!(),
        };
        let z = match (self.x, self.y, self.z) {
            (Transformation::ZPos, _, _) => Transformation::XPos,
            (Transformation::ZNeg, _, _) => Transformation::XNeg,
            (_, Transformation::ZPos, _) => Transformation::YPos,
            (_, Transformation::ZNeg, _) => Transformation::YNeg,
            (_, _, Transformation::ZPos) => Transformation::ZPos,
            (_, _, Transformation::ZNeg) => Transformation::ZNeg,
            _ => unreachable!(),
        };
        Orientation { x, y, z }
    }
}

impl Add for &Orientation {
    type Output = Orientation;

    fn add(self, rhs: Self) -> Self::Output {
        macro_rules! dim {
            ($dim:expr) => {
                match $dim {
                    Transformation::XPos => rhs.x,
                    Transformation::XNeg => rhs.x.inverse(),
                    Transformation::YPos => rhs.y,
                    Transformation::YNeg => rhs.y.inverse(),
                    Transformation::ZPos => rhs.z,
                    Transformation::ZNeg => rhs.z.inverse(),
                }
            };
        }
        Orientation {
            x: dim!(self.x),
            y: dim!(self.y),
            z: dim!(self.z),
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            x: Transformation::XPos,
            y: Transformation::YPos,
            z: Transformation::ZPos,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Transformation {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
}

impl Transformation {
    fn inverse(&self) -> Self {
        match self {
            Transformation::XPos => Transformation::XNeg,
            Transformation::XNeg => Transformation::XPos,
            Transformation::YPos => Transformation::YNeg,
            Transformation::YNeg => Transformation::YPos,
            Transformation::ZPos => Transformation::ZNeg,
            Transformation::ZNeg => Transformation::ZPos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        let (part1, part2) = solve(input);
        assert_eq!(part1.to_string(), "79");
        assert_eq!(part2.to_string(), "3621");
    }
}
