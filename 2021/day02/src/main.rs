fn main() {
    let input = common::read_input!();
    let orders: Vec<_> = input.lines().map(parse_order).collect();

    let first = first_problem(&orders);
    println!("first: {first}");

    let second = second_problem(&orders);
    println!("second: {second}");
}

#[derive(Debug)]
enum Order {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn parse_order(s: &str) -> Order {
    let components: Vec<&str> = s.split_whitespace().collect();
    let [dir, unit] = components[..] else {
            panic!("Couldn't parse {s}");
        };

    let unit: i32 = unit.parse().expect("Got a unit that was unexpected {unit}");
    match dir {
        "up" => Order::Up(unit),
        "down" => Order::Down(unit),
        "forward" => Order::Forward(unit),
        _ => panic!("Got a direction we have no clue what to do with {dir}"),
    }
}

fn first_problem(orders: &[Order]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for order in orders {
        match order {
            Order::Up(up) => depth -= up,
            Order::Down(down) => depth += down,
            Order::Forward(forward) => horizontal += forward,
        }
    }

    depth * horizontal
}

fn second_problem(orders: &[Order]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for order in orders {
        match order {
            Order::Up(x) => aim += x,
            Order::Down(x) => aim -= x,
            Order::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
        }
    }

    (depth * horizontal).abs()
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let orders: Vec<_> = input.lines().map(parse_order).collect();
    let result = first_problem(&orders);
    assert_eq!(result, 150);
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let orders: Vec<_> = input.lines().map(parse_order).collect();
    let result = second_problem(&orders);
    assert_eq!(result, 900);
}
