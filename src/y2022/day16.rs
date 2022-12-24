use std::{
    collections::{hash_map::DefaultHasher, BinaryHeap, HashSet},
    hash::{Hash, Hasher},
    str::FromStr,
};

use itertools::Itertools;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let volcano = input.parse::<Volcano>().unwrap();

    let part1 = volcano.best_path(1, 0);
    let part2 = volcano.best_path(2, 4);

    (part1, part2)
}

#[derive(Debug)]
struct Volcano {
    valves: Vec<Valve>,
    start_valve: ValveRef,
    move_times: Vec<Vec<i64>>,
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i64,
    connections: Vec<ValveRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveRef(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    actors: Vec<ActorState>,
    time: i64,
    opened: Vec<bool>,
    score: i64,
    h: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ActorState {
    target_position: ValveRef,
    time_until_target: i64,
}

impl Volcano {
    fn best_path(&self, actors: usize, initial_time: i64) -> i64 {
        let actors = (0..actors)
            .map(|_| ActorState {
                target_position: self.start_valve,
                time_until_target: 0,
            })
            .collect();
        let initial_opened = self.valves.iter().map(|v| v.flow_rate == 0).collect();
        let initial_state = State {
            actors,
            time: initial_time,
            opened: initial_opened,
            score: 0,
            h: i64::MAX,
        };

        let mut best_score = 0;
        let mut queue = BinaryHeap::new();
        let mut seen = HashSet::new();
        queue.push(initial_state);
        while let Some(state) = queue.pop() {
            if state.time > 30 {
                continue;
            }
            if !seen.insert(state.calculate_hash()) {
                continue;
            }
            if state.score > best_score {
                best_score = state.score;
                queue.retain(|s| s.h > best_score);
                // println!("{:?}", state);
            }
            if state.opened.iter().all(|v| *v) {
                break;
            }
            let moves = self.moves(&state);
            for m in moves {
                if m.h < best_score {
                    continue;
                }
                queue.push(m);
            }
        }

        best_score
    }

    fn moves(&self, state: &State) -> Vec<State> {
        // Could use some work. Move more stuff into State functions, use less of a hack
        // for the final move etc.

        let mut moves = Vec::new();
        for (actor_index, actor) in state
            .actors
            .iter()
            .enumerate()
            .filter(|(_, a)| a.time_until_target == 0)
        {
            let mut opened = state.opened.clone();
            let mut score = state.score;
            if !state.opened[actor.target_position.0] {
                let actor_valve = &self.valves[actor.target_position.0];
                score = state.score + actor_valve.flow_rate * (30 - state.time);

                opened[actor.target_position.0] = true;
            }

            for target_valve_ref in opened
                .iter()
                .enumerate()
                // .filter(|(i, v)| !**v && !state.actors.iter().any(|a| a.target_position.0 == *i))
                .filter(|(_, v)| !**v)
                .map(|(i, _)| ValveRef(i))
            {
                let mut actors = state.actors.clone();
                actors[actor_index].target_position = target_valve_ref;
                actors[actor_index].time_until_target =
                    self.move_time(actor.target_position, target_valve_ref) + 1;
                // let time_at_target = state.time + actors[actor_index].time_until_target;

                let move_time = actors.iter().map(|a| a.time_until_target).min().unwrap();
                actors
                    .iter_mut()
                    .for_each(|a| a.time_until_target -= move_time);

                let time = state.time + move_time;
                if time > 30 {
                    continue;
                }

                moves.push(State::new(self, actors, time, opened.clone(), score));
            }
        }

        if moves.is_empty() {
            if let Some(opened_index) = state.opened.iter().position(|v| !*v) {
                if state
                    .actors
                    .iter()
                    .any(|a| a.target_position.0 == opened_index)
                    && state.opened.iter().filter(|v| !*v).count() == 1
                {
                    let valve = &self.valves[opened_index];
                    let score = state.score + valve.flow_rate * (30 - state.time);
                    moves.push(State::new(
                        self,
                        state.actors.clone(),
                        state.time,
                        vec![true; state.opened.len()],
                        score,
                    ));
                }
            }
        }

        moves
    }

    fn move_time(&self, a: ValveRef, b: ValveRef) -> i64 {
        self.move_times[a.0][b.0]
    }
}

impl State {
    fn new(
        volcano: &Volcano,
        actors: Vec<ActorState>,
        time: i64,
        opened: Vec<bool>,
        score: i64,
    ) -> State {
        let h = score
            + opened
                .iter()
                .zip(volcano.valves.iter())
                .filter(|(v, _)| !**v)
                .map(|(_, valve)| valve.flow_rate * (30 - time))
                .sum::<i64>();
        State {
            actors,
            time,
            opened,
            score,
            h,
        }
    }

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.actors.iter().sorted().collect::<Vec<_>>().hash(&mut s);
        self.time.hash(&mut s);
        self.opened.hash(&mut s);
        s.finish()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.h.partial_cmp(&other.h)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Volcano {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|l| {
                let l = l.strip_prefix("Valve ").unwrap();
                let (name, l) = l.split_once(' ').unwrap();
                let l = l.strip_prefix("has flow rate=").unwrap();
                let (flow_rate, l) = l.split_once(';').unwrap();
                // lol
                let connections = l
                    .trim_start_matches(" tunnels lead")
                    .trim_start_matches(" tunnel leads")
                    .trim_start_matches(" to valve")
                    .trim_start_matches('s')
                    .trim_start()
                    .split(", ")
                    .collect::<Vec<_>>();

                (name, flow_rate, connections)
            })
            .collect::<Vec<_>>();

        let mut valves = data
            .iter()
            .map(|(name, flow_rate, _)| Valve {
                name: name.to_string(),
                flow_rate: flow_rate.parse::<i64>().unwrap(),
                connections: Vec::new(),
            })
            .collect::<Vec<_>>();

        for (name, _, connections) in data.iter() {
            let connections = connections
                .iter()
                .map(|c| ValveRef(valves.iter().position(|v| v.name == *c).unwrap()))
                .collect();
            let mut valve = valves.iter_mut().find(|v| v.name == *name).unwrap();
            valve.connections = connections;
        }

        let start_valve = ValveRef(valves.iter().position(|v| v.name == "AA").unwrap());

        let move_times = (0..valves.len())
            .map(|i| calculate_times(&valves, i))
            .collect();

        Ok(Volcano {
            valves,
            start_valve,
            move_times,
        })
    }
}

fn calculate_times(valves: &[Valve], from: usize) -> Vec<i64> {
    struct State {
        position: ValveRef,
        time: i64,
    }

    let initial_state = State {
        position: ValveRef(from),
        time: 0,
    };
    let mut queue = vec![initial_state];
    let mut best = vec![i64::MAX; valves.len()];
    while let Some(state) = queue.pop() {
        if best[state.position.0] < state.time {
            continue;
        }
        best[state.position.0] = state.time;
        let valve = &valves[state.position.0];
        for target in valve.connections.iter() {
            queue.push(State {
                position: *target,
                time: state.time + 1,
            });
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 1651.to_string());
        assert_eq!(result.1.to_string(), 1707.to_string());
    }
}
