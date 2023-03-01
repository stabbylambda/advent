use nom::{
    character::complete::{char, u32},
    multi::separated_list1,
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

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(char(' '), u32)(input);

    result.unwrap().1
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}
impl Node {
    fn sum_metadata(&self) -> u32 {
        let self_metadata: u32 = self.metadata.iter().sum();
        let child_metadata: u32 = self.children.iter().map(|x| x.sum_metadata()).sum();

        self_metadata + child_metadata
    }

    fn child(&self, child_idx: &u32) -> u32 {
        let child_idx = *child_idx as usize - 1;

        self.children
            .get(child_idx)
            .map(|x| x.value())
            .unwrap_or_default()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|child_idx| self.child(child_idx))
                .sum()
        }
    }
}

fn make_node(v: &[u32]) -> (usize, Node) {
    let child_nodes = v[0];
    let metadata_nodes = v[1];

    let mut children = vec![];
    let mut index = 2;
    for _n in 0..child_nodes {
        let (length, child) = make_node(&v[index..]);

        index += length;
        children.push(child);
    }

    let mut metadata = vec![];
    for _n in 0..metadata_nodes {
        metadata.push(v[index]);
        index += 1;
    }

    (index, Node { children, metadata })
}

fn problem1(input: &Input) -> u32 {
    let (len, node) = make_node(input);
    assert!(len == input.len());
    node.sum_metadata()
}

fn problem2(input: &Input) -> u32 {
    let (len, node) = make_node(input);
    assert!(len == input.len());
    node.value()
}

#[cfg(test)]
mod test {
    use crate::{make_node, parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 138)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 66)
    }
}
