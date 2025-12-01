use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i64 as nom_i64, newline},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Ingredient>;

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn score(&self, meal_replacement: bool) -> Option<i64> {
        let good_cookie = self.capacity >= 0
            && self.durability >= 0
            && self.flavor >= 0
            && self.texture >= 0
            && self.calories >= 0
            && (!meal_replacement || self.calories == 500);

        good_cookie.then_some(self.capacity * self.durability * self.flavor * self.texture)
    }
}

impl Add for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Sum for Ingredient {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap()
    }
}

impl Mul<i64> for &Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i64) -> Self::Output {
        Ingredient {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

fn component<'a>(name: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, i64> {
    move |input| delimited(tag(format!("{name} ").as_str()), nom_i64, opt(tag(", "))).parse(input)
}

fn parse(input: &str) -> Input {
    let ingredient = map(
        separated_pair(
            alpha1,
            tag(": "),
            (
                component("capacity"),
                component("durability"),
                component("flavor"),
                component("texture"),
                component("calories"),
            ),
        ),
        |(_name, (capacity, durability, flavor, texture, calories))| Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    );
    let result: IResult<&str, Input> = separated_list1(newline, ingredient).parse(input);

    result.unwrap().1
}

fn get_mix(ingredients: &[Ingredient], total: i64) -> Vec<Ingredient> {
    let current = &ingredients[0];
    if ingredients.len() == 1 {
        return vec![current * total];
    }

    (1..total)
        .flat_map(|n| {
            get_mix(&ingredients[1..], total - n)
                .into_iter()
                .map(|m| (current * n) + m)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn problem(input: &Input, meal_replacement: bool) -> i64 {
    let mut result = 0;
    for mix in get_mix(&input[..], 100) {
        if let Some(score) = mix.score(meal_replacement) {
            result = result.max(score);
        }
    }
    result
}

fn problem1(input: &Input) -> i64 {
    problem(input, false)
}
fn problem2(input: &Input) -> i64 {
    problem(input, true)
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 62842880)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 57600000)
    }
}
