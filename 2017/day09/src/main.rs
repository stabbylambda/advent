fn main() {
    let input = common::read_input!();

    let (answer, trash_chars) = problem(input);
    println!("problem 1 answer: {answer}");
    println!("problem 2 score: {trash_chars}");
}

fn problem(input: &str) -> (u32, u32) {
    let mut chars = input.chars();
    let mut depth = 0;
    let mut score = 0;
    let mut trash_chars = 0;
    let mut in_trash = false;

    while let Some(c) = chars.next() {
        match c {
            '!' => {
                // cancel the next character
                chars.next();
            }
            '{' if !in_trash => {
                depth += 1;
            }
            '}' if !in_trash => {
                score += depth;
                depth -= 1;
            }
            '<' if in_trash => {
                trash_chars += 1;
            }
            '<' => {
                in_trash = true;
            }
            '>' => {
                in_trash = false;
            }
            _ if in_trash => {
                trash_chars += 1;
            }
            _ => {}
        }
    }

    (score, trash_chars)
}

#[cfg(test)]
mod test {
    use crate::problem;
    #[test]
    fn first() {
        let cases = [
            (r#"{}"#, 1),
            (r#"{{{}}}"#, 6),
            (r#"{{},{}}"#, 5),
            (r#"{{{},{},{{}}}}"#, 16),
            (r#"{<a>,<a>,<a>,<a>}"#, 1),
            (r#"{{<ab>},{<ab>},{<ab>},{<ab>}}"#, 9),
            (r#"{{<!!>},{<!!>},{<!!>},{<!!>}}"#, 9),
            (r#"{{<a!>},{<a!>},{<a!>},{<ab>}}"#, 3),
        ];
        for (input, expected) in cases {
            let (result, _) = problem(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn second() {
        let cases = [
            (r#"<>"#, 0),
            (r#"<random characters>"#, 17),
            (r#"<<<<>"#, 3),
            (r#"<{!>}>"#, 2),
            (r#"<!!>"#, 0),
            (r#"<!!!>>"#, 0),
            (r#"<{o"i!a,<{i<a>"#, 10),
        ];
        for (input, expected) in cases {
            let (_, result) = problem(input);
            assert_eq!(result, expected);
        }
    }
}
