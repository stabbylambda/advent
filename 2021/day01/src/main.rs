fn main() {
    let input = common::read_input!();
    let input: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let first = first_problem(&input);
    println!("first result: {first}");

    let second = second_problem(&input);
    println!("second result: {second}");
}

fn first_problem(measurements: &[i32]) -> usize {
    measurements
        .windows(2)
        .map(|pair| if let [x, y] = pair { y > x } else { false })
        .filter(|x| *x)
        .count()
}

fn second_problem(measurements: &[i32]) -> usize {
    let triples: Vec<i32> = measurements
        .windows(3)
        .map(|triple| if let [a, b, c] = triple { a + b + c } else { 0 })
        .collect();

    first_problem(&triples)
}

#[test]
fn example1() {
    let input = include_str!("../test.txt");
    let input: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let result = first_problem(&input);
    assert_eq!(result, 7);
}

#[test]
fn example2() {
    let input = include_str!("../test.txt");
    let input: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let result = second_problem(&input);
    assert_eq!(result, 5);
}
