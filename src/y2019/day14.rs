use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let reactions = input.lines().map(Reaction::from_str).collect::<Vec<_>>();

    // Check that each chemical is the product of only one reaction
    let mut seen = HashSet::new();
    for r in reactions.iter() {
        assert!(!seen.contains(&r.output.name));
        seen.insert(r.output.name.clone());
    }

    let reaction_map = reactions
        .iter()
        .cloned()
        .map(|r| (r.output.name, (r.output.amount, r.inputs)))
        .collect::<HashMap<_, _>>();

    let mut reaction_reqs = HashMap::<&str, Vec<&str>>::new();
    for r in reactions.iter() {
        for input in r.inputs.iter() {
            reaction_reqs
                .entry(&input.name)
                .or_default()
                .push(&r.output.name);
        }
    }
    let mut counts = HashMap::new();
    counts.insert("FUEL", 1);
    reaction_reqs.insert("FUEL", vec![]);
    let part1;
    loop {
        let name = *reaction_reqs.iter().find(|(_, v)| v.is_empty()).unwrap().0;
        reaction_reqs.remove(name).unwrap();
        let needed = counts.remove(name).unwrap();
        if name == "ORE" {
            part1 = needed;
            break;
        }
        let (output_amount, inputs) = reaction_map.get(name).unwrap();

        let mut n = needed / output_amount;
        if needed % output_amount != 0 {
            n += 1;
        }

        for input in inputs.iter() {
            *counts.entry(&input.name).or_default() += input.amount * n;
        }

        reaction_reqs
            .iter_mut()
            .for_each(|(_, v)| v.retain(|u| *u != name));
    }

    (part1, 0)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Reaction {
    inputs: Vec<Unit>,
    output: Unit,
}

impl Reaction {
    fn from_str(s: &str) -> Self {
        let (inputs, output) = s.split_once(" => ").unwrap();
        Self {
            inputs: inputs.split(", ").map(Unit::from_str).collect(),
            output: Unit::from_str(output),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Unit {
    name: String,
    amount: i64,
}

impl Unit {
    fn from_str(s: &str) -> Self {
        let (num, name) = s.split_once(' ').unwrap();
        Self {
            name: name.to_string(),
            amount: num.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let (part1, _) = solve(input);
        assert_eq!(part1.to_string(), "165");

        let input = r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let (part1, _) = solve(input);
        assert_eq!(part1.to_string(), "2210736");
    }
}
