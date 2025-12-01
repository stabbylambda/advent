use nom::{
    branch::alt,
    character::complete::{alphanumeric1, char, newline},
    combinator::map,
    multi::{many1, separated_list0},
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

type Input = Vec<Address>;

#[derive(Debug, PartialEq, Eq)]
enum AddressPart {
    Supernet(Vec<char>),
    Hypernet(Vec<char>),
}

impl AddressPart {
    fn chars(&self) -> &[char] {
        match self {
            AddressPart::Supernet(cs) => cs,
            AddressPart::Hypernet(cs) => cs,
        }
    }
    fn has_abba(&self) -> bool {
        self.chars()
            .windows(4)
            .any(|w| w[0] != w[1] && w[0] == w[3] && w[1] == w[2])
    }

    fn get_abas(&self) -> impl Iterator<Item = (&char, &char)> {
        self.chars().windows(3).filter_map(|x| {
            let [a1, b, a2] = x else { return None };

            (a1 == a2 && a1 != b).then_some((a1, b))
        })
    }

    fn has_bab(&self, (aba_a, aba_b): (&char, &char)) -> bool {
        self.get_abas()
            .any(|(bab_b, bab_a)| aba_a == bab_a && aba_b == bab_b)
    }
}

#[derive(Debug)]
struct Address {
    parts: Vec<AddressPart>,
}
impl Address {
    fn hypernet_parts(&self) -> impl Iterator<Item = &AddressPart> {
        self.parts
            .iter()
            .filter(|part| matches!(part, AddressPart::Hypernet { .. }))
    }
    fn supernet_parts(&self) -> impl Iterator<Item = &AddressPart> {
        self.parts
            .iter()
            .filter(|part| matches!(part, AddressPart::Supernet { .. }))
    }

    fn supports_tls(&self) -> bool {
        let valid_super = self.supernet_parts().any(|part| part.has_abba());
        let valid_hyper = self.hypernet_parts().all(|part| !part.has_abba());

        valid_super && valid_hyper
    }

    fn supports_ssl(&self) -> bool {
        self.supernet_parts()
            .flat_map(|s| s.get_abas())
            .any(|aba| self.hypernet_parts().any(|h| h.has_bab(aba)))
    }

    fn parse(s: &str) -> IResult<&str, Self> {
        let part = |x| map(alphanumeric1, |x: &str| x.chars().collect::<Vec<char>>()).parse(x);

        map(
            many1(alt((
                map(part, AddressPart::Supernet),
                map(delimited(char('['), part, char(']')), AddressPart::Hypernet),
            ))),
            |parts| Address { parts },
        )
        .parse(s)
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list0(newline, Address::parse).parse(input);
    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    input.iter().filter(|x| x.supports_tls()).count()
}

fn problem2(input: &Input) -> usize {
    input.iter().filter(|x| x.supports_ssl()).count()
}

#[cfg(test)]
mod test {
    use crate::Address;
    #[test]
    fn first() {
        let tests = [
            ("abba[mnop]qrst", true, "supports TLS (abba outside square brackets)"),
            ("abcd[bddb]xyyx", false, "does not support TLS (bddb is within square brackets, even though xyyx is outside square brackets)"),
            ("aaaa[qwer]tyui", false, "does not support TLS (aaaa is invalid; the interior characters must be different)"),
            ("ioxxoj[asdfgh]zxcvbn", true,  "supports TLS (oxxo is outside square brackets, even though it's within a larger string)"),
        ];

        for (input, expected, reason) in tests {
            let result = Address::parse(input).unwrap().1;
            assert_eq!(result.supports_tls(), expected, "{reason}");
        }
    }

    #[test]
    fn second() {
        let tests = [
            ("aba[bab]xyz", true, "supports SSL (aba outside square brackets with corresponding bab within square brackets)."),
            ("xyx[xyx]xyx", false, "does not support SSL (xyx, but no corresponding yxy)."),
            ("aaa[kek]eke", true, "supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related, because the interior character must be different)."),
            ("zazbz[bzb]cdb", true, "supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz and zbz overlap)."),
        ];

        for (input, expected, reason) in tests {
            let result = Address::parse(input).unwrap().1;
            assert_eq!(result.supports_ssl(), expected, "{reason}");
        }
    }
}
