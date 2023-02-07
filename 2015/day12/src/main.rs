use common::get_raw_input;

use serde_json::Value;

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Value;

fn parse(input: &str) -> Input {
    serde_json::from_str(input).unwrap()
}

fn sum_all_numbers(v: &Value, exclude_red: bool) -> i64 {
    match v {
        Value::Number(x) => x.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|x| sum_all_numbers(x, exclude_red)).sum(),
        Value::Object(o) => {
            if exclude_red && o.values().any(|x| x == "red") {
                0
            } else {
                o.values().map(|x| sum_all_numbers(x, exclude_red)).sum()
            }
        }
        _ => 0,
    }
}

fn problem1(input: &Input) -> i64 {
    sum_all_numbers(input, false)
}

fn problem2(input: &Input) -> i64 {
    sum_all_numbers(input, true)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = parse(r#"{"a":{"b":4},"c":-1}"#);
        let result = problem1(&input);
        assert_eq!(result, 3)
    }

    #[test]
    fn second() {
        let input = parse(r#"{"d":"red","e":[1,2,3,4],"f":5}"#);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
