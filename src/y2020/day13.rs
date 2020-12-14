pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse::<u64>().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(|a| {
            if a == "x" {
                None
            } else {
                Some(a.parse::<u64>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    let (bus_id, depart_time) = (earliest..)
        .find_map(|time| {
            bus_ids
                .iter()
                .filter_map(|b| b.as_ref())
                .find(|bus_id| time % **bus_id == 0)
                .map(|bus_id| (bus_id, time))
        })
        .unwrap();
    let wait_time = depart_time - earliest;
    let part1 = bus_id * wait_time;

    // https://math.stackexchange.com/a/1485998
    // Assume all numbers are co-prime
    let final_mod = bus_ids.iter().filter_map(|id| id.as_ref()).product::<u64>();
    let numbered_ids = bus_ids
        .iter()
        .enumerate()
        // want t to = -offset % bus_id
        .filter_map(|(i, id)| id.map(|v| ((-(i as i64)).rem_euclid(v as i64), v)))
        .collect::<Vec<_>>();
    println!("{:?}", numbered_ids);
    // Skip e0 as offset == 0
    let part2 = numbered_ids
        .iter()
        .skip(1)
        .map(|(offset, bus_id)| {
            let a = numbered_ids
                .iter()
                .filter(|(i, _)| i != offset)
                .map(|(_, id)| id)
                .product::<u64>();
            // a * x = 1 (mod bus_id)
            let x = (1..).find(|x| (a * x) % bus_id == 1).unwrap();
            let e = (a * x) % final_mod;
            *offset as u64 * e
        })
        .sum::<u64>()
        % final_mod;

    (part1, part2)
}
