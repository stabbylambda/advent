use common::{answer, digits};

fn main() {
    let input = 440231;

    answer!(problem1(input));
    answer!(problem2(input));
}

fn problem1(input: usize) -> u64 {
    let mut recipes: Vec<u8> = vec![3, 7];

    let mut elf1 = 0;
    let mut elf2 = 1;

    while recipes.len() < input + 10 {
        let recipe1 = recipes[elf1];
        let recipe2 = recipes[elf2];

        let score = recipe1 + recipe2;
        if score >= 10 {
            recipes.push(score / 10);
            recipes.push(score % 10);
        } else {
            recipes.push(score)
        }

        elf1 = (elf1 + 1 + recipe1 as usize) % recipes.len();
        elf2 = (elf2 + 1 + recipe2 as usize) % recipes.len();
    }

    let s: String = recipes[input..input + 10]
        .iter()
        .map(|x| x.to_string())
        .collect();

    s.parse::<u64>().unwrap()
}

fn problem2(input: usize) -> usize {
    let check = digits(input);
    let check_len = check.len();
    let check = &check[..];
    let mut recipes: Vec<u8> = vec![3, 7];

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let recipe1 = recipes[elf1];
        let recipe2 = recipes[elf2];

        let score = recipe1 + recipe2;
        if score >= 10 {
            recipes.push(score / 10);
            recipes.push(score % 10);
        } else {
            recipes.push(score)
        }

        elf1 = (elf1 + 1 + recipe1 as usize) % recipes.len();
        elf2 = (elf2 + 1 + recipe2 as usize) % recipes.len();

        let recipe_count = recipes.len();

        if recipe_count > check_len && &recipes[recipe_count - check_len..recipe_count] == check {
            return recipe_count - check_len;
        } else if score >= 10 {
            let recipe_count = recipe_count - 1;
            if recipe_count > check_len && &recipes[recipe_count - check_len..recipe_count] == check
            {
                return recipe_count - check_len;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let cases = [
            (9, 5158916779),
            (5, 124515891),
            (18, 9251071085),
            (2018, 5941429882),
        ];
        for (input, expected) in cases {
            let result = problem1(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn second() {
        let cases = [(51589, 9), (92510, 18), (59414, 2018)];
        for (input, expected) in cases {
            let result = problem2(input);
            assert_eq!(result, expected);
        }
    }
}
