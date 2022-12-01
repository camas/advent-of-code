use std::{sync::mpsc, thread};

use itertools::Itertools;

use super::intcode::{Handler, Machine};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let machine = Machine::from_str(input);

    let mut best = 0;
    for values in (0..=4).permutations(5) {
        let (mut senders, mut receivers) = (0..6)
            .map(|_| mpsc::channel::<i64>())
            .unzip::<_, _, Vec<_>, Vec<_>>();

        // Phase settings
        for (i, v) in values.iter().enumerate() {
            senders[i].send(*v).unwrap();
        }

        // Take first sender and last receiver as input/output channels
        let input = senders.remove(0);
        let output = receivers.pop().unwrap();

        // Create handlers
        let handlers = senders
            .into_iter()
            .zip(receivers.into_iter())
            .map(|(s, r)| AmpHandler {
                input_rx: r,
                output_tx: s,
            })
            .collect::<Vec<_>>();

        // Run machines
        for mut handler in handlers {
            let mut machine = machine.clone();
            thread::spawn(move || {
                machine.run(Some(&mut handler));
            });
        }

        // Send start signal
        input.send(0).unwrap();

        // Receive output
        let final_value = output.recv().unwrap();
        if final_value > best {
            best = final_value;
        }
    }
    let part1 = best;

    let mut best = 0;
    for values in (5..=9).permutations(5) {
        let (mut senders, mut receivers) = (0..6)
            .map(|_| mpsc::channel::<i64>())
            .unzip::<_, _, Vec<_>, Vec<_>>();

        // Phase settings
        for (i, v) in values.iter().enumerate() {
            senders[i].send(*v).unwrap();
        }

        // Take first sender and last receiver as input/output channels
        let input = senders.remove(0);
        let output = receivers.pop().unwrap();

        // Create handlers
        let handlers = senders
            .into_iter()
            .zip(receivers.into_iter())
            .map(|(s, r)| AmpHandler {
                input_rx: r,
                output_tx: s,
            })
            .collect::<Vec<_>>();

        // Run machines
        let mut threads = Vec::new();
        for mut handler in handlers {
            let mut machine = machine.clone();
            threads.push(thread::spawn(move || {
                machine.run(Some(&mut handler));
            }));
        }

        // Send start signal
        input.send(0).unwrap();

        // Send output to input until machines halt
        let mut last = 0;
        while let Ok(value) = output.recv() {
            let _ = input.send(value);
            last = value;
        }
        if last > best {
            best = last;
        }
    }
    let part2 = best;

    (part1, part2)
}

struct AmpHandler {
    input_rx: mpsc::Receiver<i64>,
    output_tx: mpsc::Sender<i64>,
}

impl Handler for AmpHandler {
    fn input(&mut self, _: &Machine) -> i64 {
        self.input_rx.recv().unwrap()
    }

    fn output(&mut self, _: &Machine, value: i64) {
        self.output_tx.send(value).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = r"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let (part1, _) = solve(input);
        assert_eq!(part1.to_string(), "43210");
    }
}
