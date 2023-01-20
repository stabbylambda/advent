use std::fmt::Debug;
use std::str::FromStr;

pub fn get_raw_input() -> String {
    crate::util::get_raw_input("./test.txt")
}
pub fn get_input<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    crate::util::get_input("./test.txt", f)
}

pub fn get_input_strings() -> Vec<String> {
    get_input(|s| s.to_owned())
}

pub fn get_numbers<T>() -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    crate::util::get_numbers("./test.txt")
}
