use nom::{
    bytes::complete::tag,
    character::complete::{i32 as nom_i32, newline},
    combinator::map,
    sequence::{delimited, preceded},
    IResult, Parser,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Boss {
    pub hit_points: i32,
    pub damage: i32,
}
impl Boss {
    pub fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }
    pub fn parse(input: &str) -> Boss {
        let result: IResult<&str, Boss> = map(
            (
                delimited(tag("Hit Points: "), nom_i32, newline),
                preceded(tag("Damage: "), nom_i32),
            ),
            |(hit_points, damage)| Boss { hit_points, damage },
        )
        .parse(input);

        result.unwrap().1
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spell {
    pub name: &'static str,
    pub cost: i32,
    pub mana: i32,
    pub armor: i32,
    pub healing: i32,
    pub damage: i32,
    pub turns: i32,
}
impl Spell {
    pub const fn new(
        name: &'static str,
        cost: i32,
        mana: i32,
        armor: i32,
        healing: i32,
        damage: i32,
        turns: i32,
    ) -> Spell {
        Spell {
            name,
            cost,
            mana,
            armor,
            healing,
            damage,
            turns,
        }
    }

    pub fn is_immediate(&self) -> bool {
        self.turns == 0
    }
}
