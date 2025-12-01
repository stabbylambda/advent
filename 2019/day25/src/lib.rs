use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{newline, u32},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

const PRESSURE_SENSITIVE :&str = "\n\n\n== Pressure-Sensitive Floor ==\nAnalyzing...\n\nDoors here lead:\n- east\n\nA loud, robotic voice says ";
const ALERT: &str = r#""Alert! Droids on this ship are "#;
const EJECTED: &str = " than the detected value!\" and you are ejected back to the checkpoint.\n";
const ANALYSIS_COMPLETE: &str = "\"Analysis complete! You may proceed.\" and you enter the cockpit.\nSanta notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.\n\"Oh, hello! You should be able to get in by typing ";
const ON_THE_KEYPAD: &str = " on the keypad at the main airlock.\"\n";
const COMMAND: &str = ".\n\nCommand?\n";

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub doors: Vec<Door>,
    pub items: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PressureSensorResult {
    Lighter,
    Heavier,
    Password(u32),
}

#[derive(Debug)]
pub enum Output {
    PressureSensor(PressureSensorResult, Option<Room>),
    InspectRoom(Room),
    TakeItem(String),
    DropItem(String),
    SomethingElse(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Door {
    North,
    South,
    East,
    West,
}

impl Display for Door {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Door::North => write!(f, "north"),
            Door::South => write!(f, "south"),
            Door::East => write!(f, "east"),
            Door::West => write!(f, "west"),
        }
    }
}

fn parse_pressure_sensor(s: &str) -> IResult<&str, (PressureSensorResult, Option<Room>)> {
    (
        preceded(
            tag(PRESSURE_SENSITIVE),
            alt((
                delimited(
                    tag(ALERT),
                    alt((
                        map(tag("lighter"), |_| PressureSensorResult::Lighter),
                        map(tag("heavier"), |_| PressureSensorResult::Heavier),
                    )),
                    tag(EJECTED),
                ),
                delimited(
                    tag(ANALYSIS_COMPLETE),
                    map(u32, PressureSensorResult::Password),
                    tag(ON_THE_KEYPAD),
                ),
            )),
        ),
        opt(parse_room),
    ).parse(s)
}

fn parse_door(s: &str) -> IResult<&str, Door> {
    alt((
        map(tag("north"), |_| Door::North),
        map(tag("south"), |_| Door::South),
        map(tag("east"), |_| Door::East),
        map(tag("west"), |_| Door::West),
    )).parse(s)
}

fn parse_room(s: &str) -> IResult<&str, Room> {
    map(
        (
            delimited(
                tag("\n\n\n== "),
                map(take_until(" =="), |x: &str| x.to_string()),
                tag(" ==\n"),
            ),
            terminated(take_until("\n"), tag("\n\n")),
            preceded(
                tag("Doors here lead:\n"),
                separated_list1(newline, preceded(tag("- "), parse_door)),
            ),
            opt(preceded(
                tag("\n\nItems here:\n"),
                separated_list1(newline, preceded(tag("- "), take_until("\n"))),
            )),
        ),
        |(name, _description, doors, items)| {
            let items = items.unwrap_or_default();
            Room {
                name,
                doors,
                items: items.iter().map(|x| x.to_string()).collect(),
            }
        },
    ).parse(s)
}

fn parse_take(s: &str) -> IResult<&str, String> {
    delimited(
        tag("\nYou take the "),
        map(take_until("."), |x: &str| x.to_string()),
        tag(COMMAND),
    ).parse(s)
}
fn parse_drop(s: &str) -> IResult<&str, String> {
    delimited(
        tag("\nYou drop the "),
        map(take_until("."), |x: &str| x.to_string()),
        tag(COMMAND),
    ).parse(s)
}

pub fn parse_output(s: &str) -> IResult<&str, Output> {
    alt((
        map(parse_pressure_sensor, |(p, r)| Output::PressureSensor(p, r)),
        map(parse_room, Output::InspectRoom),
        map(parse_take, Output::TakeItem),
        map(parse_drop, Output::DropItem),
    )).parse(s)
}

#[test]
fn test_room() {
    let output = "\n\n\n== Hull Breach ==\nYou got in through a hole in the floor here. To keep your ship from also freezing, the hole has been sealed.\n\nDoors here lead:\n- north\n- south\n- west\n\nCommand?\n";

    let room = parse_room(output).unwrap().1;
    assert_eq!(room.name, "Hull Breach");
    assert_eq!(room.doors, vec![Door::North, Door::South, Door::West]);
    assert!(room.items.is_empty());
}
#[test]
fn test_pressure() {
    let s = "\n\n\n== Pressure-Sensitive Floor ==\nAnalyzing...\n\nDoors here lead:\n- east\n\nA loud, robotic voice says \"Alert! Droids on this ship are lighter than the detected value!\" and you are ejected back to the checkpoint.\n";
    assert_eq!(
        parse_pressure_sensor(s).unwrap().1 .0,
        PressureSensorResult::Lighter
    );
}
