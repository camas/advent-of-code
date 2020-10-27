use serde_json::Value;

use crate::Exercise;

pub struct Day12;

impl Exercise for Day12 {
    fn part1(&self, input: &str) -> String {
        let root: Value = serde_json::from_str(input).unwrap();

        let mut total = 0;
        let mut q = vec![&root];
        while !q.is_empty() {
            let node = q.pop().unwrap();
            match node {
                Value::Null => {}
                Value::Bool(_) => {}
                Value::Number(value) => total += value.as_i64().unwrap(),
                Value::String(_) => {}
                Value::Array(array) => q.extend(array),
                Value::Object(object) => q.extend(object.values()),
            }
        }

        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let root: Value = serde_json::from_str(input).unwrap();

        let mut total = 0;
        let mut q = vec![&root];
        while !q.is_empty() {
            let node = q.pop().unwrap();
            match node {
                Value::Null => {}
                Value::Bool(_) => {}
                Value::Number(value) => total += value.as_i64().unwrap(),
                Value::String(_) => {}
                Value::Array(array) => q.extend(array),
                Value::Object(object) => {
                    if !object.values().any(|a| a == "red") {
                        q.extend(object.values());
                    }
                }
            }
        }

        total.to_string()
    }
}
