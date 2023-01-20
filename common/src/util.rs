use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn get_raw_input(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}

pub fn get_input<T, F>(path: &str, f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    get_raw_input(path).lines().map(f).collect()
}

pub fn get_numbers<T>(path: &str) -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let input = get_raw_input(path);
    let input = input.lines().next().unwrap();
    let input: Vec<T> = input.split(',').map(|s| s.parse::<T>().unwrap()).collect();
    input
}
