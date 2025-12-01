use std::collections::{BTreeMap, BTreeSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::delimited,
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

type Input<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

fn parse(input: &str) -> Input<'_> {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        (
            separated_list1(tag(" "), alpha1),
            delimited(
                tag(" (contains "),
                separated_list1(tag(", "), alpha1),
                tag(")"),
            ),
        ),
    ).parse(input);

    result.unwrap().1
}

type AllergenMap<'a> = BTreeMap<&'a str, BTreeSet<&'a str>>;

fn create_allergen_map<'a>(input: &'a Input) -> AllergenMap<'a> {
    let mut all_allergens: AllergenMap = BTreeMap::new();

    // get the list of ingredients that each allergen could potentially map to
    let allergen_map: Vec<(&str, BTreeSet<&str>)> = input
        .iter()
        .flat_map(|(ingredients, allergens)| {
            allergens
                .iter()
                .map(|allergen| (*allergen, ingredients.iter().copied().collect()))
        })
        .collect();

    // combine all the sets
    for (allergen, ingredients) in allergen_map {
        all_allergens
            .entry(allergen)
            .and_modify(|x| {
                *x = x.intersection(&ingredients).copied().collect();
            })
            .or_insert(ingredients);
    }

    all_allergens
}

fn problem1(input: &Input) -> usize {
    let all_ingredients: Vec<&str> = input.iter().flat_map(|x| x.0.iter().copied()).collect();
    let all_allergens = create_allergen_map(input);

    let allergen_ingredients: BTreeSet<&str> = all_allergens
        .values()
        .flat_map(|x| x.iter().copied())
        .collect();

    all_ingredients
        .iter()
        .filter(|i| !allergen_ingredients.contains(*i))
        .copied()
        .count()
}

fn problem2(input: &Input) -> String {
    let mut all_allergens = create_allergen_map(input);
    let mut dangerous: BTreeMap<&str, &str> = BTreeMap::new();

    while !all_allergens.is_empty() {
        let known: Vec<(&str, &str)> = all_allergens
            .iter()
            .filter_map(|(k, v)| (v.len() == 1).then_some((*k, *v.first().unwrap())))
            .collect();

        for (ingredient, allergen) in known {
            dangerous.insert(ingredient, allergen);
            all_allergens.remove(ingredient);

            for v in all_allergens.values_mut() {
                v.remove(allergen);
            }
        }
    }

    let dangerous: Vec<String> = dangerous.values().map(|x| x.to_string()).collect();
    dangerous.join(",")
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 5)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, "mxmxvkd,sqjhc,fvjkl".to_string())
    }
}
