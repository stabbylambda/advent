use std::{collections::HashMap, fmt::Display, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u16, u32, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<GuardRecord>;

#[derive(Debug)]
struct Sleep {
    start: Date,
    end: Date,
}

impl Sleep {
    fn minute_range(&self) -> Range<u8> {
        let start = if self.start.hour == 23 {
            0
        } else {
            self.start.minute
        };

        let end = self.end.minute;

        start..end
    }
}

#[derive(Debug)]
struct Date {
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
struct GuardRecord {
    id: u32,
    sleep_records: Vec<Sleep>,
}

impl Display for GuardRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n: u8 = 0;
        write!(f, "{:5}  ", self.id)?;

        for sleep in &self.sleep_records {
            // write them being awake until the next sleep cycle
            while n < sleep.start.minute {
                write!(f, ".")?;
                n += 1;
            }

            // now write the sleep cycle
            for x in sleep.minute_range() {
                write!(f, "#")?;
                n = x;
            }
        }

        // finish out the hour
        while n < 60 {
            write!(f, ".")?;
            n += 1;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Input {
    // Pre sort the lines. I don't want to deal with trying to combine structs later
    let mut input = input.lines().collect::<Vec<&str>>();
    input.sort();
    let sorted = input.join("\n");

    let date = |s| {
        map(
            delimited(
                char('['),
                tuple((
                    terminated(u16, char('-')),
                    terminated(u8, char('-')),
                    terminated(u8, char(' ')),
                    terminated(u8, char(':')),
                    u8,
                )),
                char(']'),
            ),
            |(_year, _month, _day, hour, minute)| Date { hour, minute },
        )(s)
    };

    let begin_shift = |s| {
        separated_pair(
            date,
            char(' '),
            delimited(tag("Guard #"), u32, tag(" begins shift")),
        )(s)
    };
    let falls_asleep = terminated(date, tag(" falls asleep"));
    let wakes_up = terminated(date, tag(" wakes up"));

    let sleep_record = map(
        separated_pair(falls_asleep, newline, wakes_up),
        |(sleep, wake)| Sleep {
            start: sleep,
            end: wake,
        },
    );

    let result: IResult<&str, Input> = separated_list1(
        newline,
        // there must be a better way to do this. some of the elves never sleep, so there's no separated_list1 of sleep records
        alt((
            map(
                separated_pair(begin_shift, newline, separated_list1(newline, sleep_record)),
                |((_begin_shift, id), sleep_records)| GuardRecord { id, sleep_records },
            ),
            map(begin_shift, |(_begin_shift, id)| GuardRecord {
                id,
                sleep_records: vec![],
            }),
        )),
    )(&sorted);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    let mut guards: HashMap<u32, Vec<u32>> = HashMap::new();
    for record in input {
        for sleep_record in &record.sleep_records {
            for n in sleep_record.minute_range() {
                guards
                    .entry(record.id)
                    .and_modify(|times_asleep| times_asleep[n as usize] += 1)
                    .or_insert_with(|| vec![0; 60]);
            }
        }
    }

    // find the guard who spent the most time asleep
    guards
        .iter()
        .max_by_key(|&(_id, times_asleep)| times_asleep.iter().sum::<u32>())
        .map(|(guard, minutes)| {
            // find the minute he spent asleep the most
            let minute = minutes
                .iter()
                .enumerate()
                .max_by_key(|(_idx, count)| *count)
                .unwrap()
                .0;
            guard * (minute as u32)
        })
        .unwrap()
}

fn problem2(input: &Input) -> u32 {
    let mut minutes: HashMap<(u8, u32), u32> = HashMap::new();

    for record in input {
        for sleep_record in &record.sleep_records {
            for n in sleep_record.minute_range() {
                minutes
                    .entry((n, record.id))
                    .and_modify(|count| *count += 1)
                    .or_default();
            }
        }
    }

    minutes
        .iter()
        .max_by_key(|&(_key, value)| value)
        .map(|((minute, id), _times)| (*minute as u32) * id)
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 240)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 4455)
    }
}
