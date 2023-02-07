use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric0, line_ending, not_line_ending, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use petgraph::{
    algo::toposort,
    graph::NodeIndex,
    Direction::{Incoming, Outgoing},
    Graph,
};
fn main() {
    let lines = include_str!("../input.txt");
    let input = Command::parse_all(lines);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Data<'a> {
    name: &'a str,
    size: u32,
}
impl<'a> Data<'a> {
    fn new(name: &'a str, size: u32) -> Data<'a> {
        Data { name, size }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Listing<'a> {
    Directory(Data<'a>),
    File(Data<'a>),
}

impl<'a> Listing<'a> {
    fn parse(s: &str) -> IResult<&str, Vec<Listing>> {
        separated_list1(
            line_ending,
            alt((
                map(
                    separated_pair(nom_u32, tag(" "), not_line_ending),
                    |(size, name)| Listing::File(Data::new(name, size)),
                ),
                preceded(
                    tag("dir "),
                    map(alphanumeric0, |x| Listing::Directory(Data::new(x, 0))),
                ),
            )),
        )(s)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Command<'a> {
    GoToRoot,
    GoUp,
    ChangeDir(&'a str),
    List(Vec<Listing<'a>>),
}

impl<'a> Command<'a> {
    fn parse(s: &str) -> IResult<&str, Command> {
        preceded(
            tag("$ "),
            alt((
                preceded(
                    tag("cd "),
                    map(not_line_ending, |x| match x {
                        ".." => Command::GoUp,
                        "/" => Command::GoToRoot,
                        _ => Command::ChangeDir(x),
                    }),
                ),
                preceded(
                    tag("ls"),
                    map(preceded(line_ending, Listing::parse), Command::List),
                ),
            )),
        )(s)
    }

    fn parse_all(s: &str) -> Vec<Command> {
        separated_list1(line_ending, Command::parse)(s).unwrap().1
    }
}

struct FileSystem<'a> {
    graph: Graph<Listing<'a>, ()>,
    root: NodeIndex,
}

impl<'a> FileSystem<'a> {
    const TOTAL: u32 = 70_000_000;

    fn build(commands: &'a [Command]) -> FileSystem<'a> {
        let mut graph: Graph<Listing, ()> = Graph::new();
        // gotta add in a slash just so we have the root index for later
        let root = graph.add_node(Listing::Directory(Data::new("/", 0)));

        let mut current = root;
        for command in commands {
            match command {
                Command::GoToRoot => current = root,
                Command::GoUp => {
                    current = graph.neighbors_directed(current, Incoming).next().unwrap();
                }
                Command::ChangeDir(dir) => {
                    let mut neighbors = graph.neighbors_directed(current, Outgoing);
                    // find the outgoing directory
                    match neighbors
                        .find(|idx| graph[*idx] == Listing::Directory(Data { name: dir, size: 0 }))
                    {
                        // if it exists, set the current
                        Some(dir_idx) => current = dir_idx,
                        //otherwise create it and set the current
                        None => {
                            let new_idx = graph.add_node(Listing::Directory(Data::new(dir, 0)));
                            graph.add_edge(current, new_idx, ());
                            current = new_idx;
                        }
                    };
                }
                Command::List(children) => {
                    // go through the children in the listing
                    for child in children {
                        // find all the outgoing neighbors
                        let mut neighbors = graph.neighbors_directed(current, Outgoing);
                        match neighbors.find(|idx| graph[*idx] == *child) {
                            // this is here just in case there's a double list
                            Some(_) => {}
                            // this node hasn't been found yet, so add it
                            None => {
                                let new_idx = graph.add_node(*child);
                                graph.add_edge(current, new_idx, ());
                            }
                        };
                    }
                }
            }
        }
        let mut fs = FileSystem { graph, root };
        fs.calculate_sizes();
        fs
    }

    fn free_space(&self) -> u32 {
        let used = match self.graph[self.root] {
            Listing::Directory(data) => data.size,
            Listing::File(data) => data.size,
        };
        FileSystem::TOTAL - used
    }

    fn calculate_sizes(&mut self) {
        let graph = &mut self.graph;
        graph.reverse();

        let t = toposort(&*graph, None).unwrap();
        for idx in t {
            if idx == self.root {
                continue;
            }

            let node = graph[idx];
            let current_size = match node {
                Listing::Directory(data) => data.size,
                Listing::File(data) => data.size,
            };

            // add our size to the parent size
            let parent_idx = graph.neighbors(idx).next().unwrap();
            let parent = &mut graph[parent_idx];
            match parent {
                Listing::Directory(data) => data.size += current_size,
                _ => panic!("adding size to a file makes no sense"),
            };
        }
    }

    fn get_directories(&self) -> Vec<Listing<'a>> {
        let t = toposort(&self.graph, None).unwrap();
        t.iter()
            .filter_map(|idx| {
                let node = self.graph[*idx];
                match node {
                    listing @ Listing::Directory(_) => Some(listing),
                    _ => None,
                }
            })
            .collect()
    }
}

fn problem1(commands: &[Command]) -> u32 {
    let filesystem = FileSystem::build(commands);
    filesystem
        .get_directories()
        .iter()
        .filter_map(|listing| match listing {
            Listing::Directory(data) if data.size <= 100_000 => Some(data.size),
            _ => None,
        })
        .sum()
}

fn problem2(commands: &[Command]) -> u32 {
    let needed = 30_000_000;

    let filesystem = FileSystem::build(commands);
    let free = filesystem.free_space();

    let mut dirs: Vec<u32> = filesystem
        .get_directories()
        .iter()
        .filter_map(|listing| match listing {
            Listing::Directory(data) if free + data.size > needed => Some(data.size),
            _ => None,
        })
        .collect();
    dirs.sort();

    dirs[0]
}

#[cfg(test)]
mod test {
    use crate::{problem1, problem2, Command, Data, Listing};
    #[test]
    fn first() {
        let lines = include_str!("../test.txt");
        let input = Command::parse_all(lines);
        let result = problem1(&input);
        assert_eq!(result, 95437)
    }

    #[test]
    fn second() {
        let lines = include_str!("../test.txt");
        let input = Command::parse_all(lines);
        let result = problem2(&input);
        assert_eq!(result, 24933642)
    }
    #[test]
    fn ls() {
        let ls = Command::parse(
            r#"$ ls
dir foo
1234 foo.txt"#,
        );
        assert_eq!(
            ls.unwrap().1,
            Command::List(vec![
                Listing::Directory(Data::new("foo", 0)),
                Listing::File(Data::new("foo.txt", 1234)),
            ])
        );
    }

    #[test]
    fn cd() {
        let cd = Command::parse("$ cd ..").unwrap().1;
        assert_eq!(cd, Command::GoUp);

        let cd = Command::parse("$ cd /").unwrap().1;
        assert_eq!(cd, Command::GoToRoot);

        let cd = Command::parse("$ cd foo").unwrap().1;
        assert_eq!(cd, Command::ChangeDir("foo"));
    }

    #[test]
    fn listing() {
        let listing = Listing::parse("dir foo").unwrap().1;
        assert_eq!(listing, vec![Listing::Directory(Data::new("foo", 0))]);

        let listing = Listing::parse(
            r#"1234 foo.txt
dir foo
5678 bar.txt"#,
        )
        .unwrap()
        .1;
        assert_eq!(
            listing,
            vec![
                Listing::File(Data::new("foo.txt", 1234)),
                Listing::Directory(Data::new("foo", 0)),
                Listing::File(Data::new("bar.txt", 5678))
            ]
        );
    }
}
