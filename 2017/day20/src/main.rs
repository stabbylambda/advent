use common::{answer, read_input};
use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult, Parser,
};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Particle>;

#[derive(Clone, Copy, Debug)]
struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}

impl Particle {
    fn update(&mut self) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn distance(&self) -> i64 {
        let (px, py, pz) = self.position;
        px.abs() + py.abs() + pz.abs()
    }
}

fn parse(input: &str) -> Input {
    let triple = |s| (terminated(i64, char(',')), terminated(i64, char(',')), i64).parse(s);
    let particle = |s| {
        map(
            (
                delimited(tag("p=<"), triple, tag(">, ")),
                delimited(tag("v=<"), triple, tag(">, ")),
                delimited(tag("a=<"), triple, tag(">")),
            ),
            |(p, v, a)| Particle {
                position: p,
                velocity: v,
                acceleration: a,
            },
        )
        .parse(s)
    };

    let result: IResult<&str, Input> = separated_list1(newline, particle).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let mut particles: Vec<Particle> = input.clone();
    let mut closest_particle = usize::MAX;

    // simulate five hundred frames, should be enough to get the acceleration to take effect
    for _n in 0..500 {
        let mut frame_min = i64::MAX;
        let mut frame_min_particle = usize::MAX - 1;

        // update all particles
        for (idx, p) in particles.iter_mut().enumerate() {
            p.update();

            // if this is the closest, then we need to keep track of it
            if p.distance() < frame_min {
                frame_min = p.distance().min(frame_min);
                frame_min_particle = idx;
            }
        }

        closest_particle = frame_min_particle;
    }

    closest_particle
}

fn problem2(input: &Input) -> usize {
    let mut particles: Vec<Particle> = input.clone();

    for _n in 0..1000 {
        for p in &mut particles {
            p.update();
        }

        let mut destroyed = vec![];
        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                let p1 = particles[i];
                let p2 = particles[j];

                if p1.position == p2.position {
                    destroyed.push(p1.position);
                }
            }
        }

        particles.retain(|x| !destroyed.contains(&x.position));
    }

    particles.len()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test1.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 0)
    }

    #[test]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1)
    }
}
