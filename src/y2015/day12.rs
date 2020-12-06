use serde_json::Value;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
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
    let part1 = total;

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
    let part2 = total;

    (part1, part2)
}
