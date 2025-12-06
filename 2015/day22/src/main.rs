use std::collections::BinaryHeap;

use advent_2015_22::{Boss, Spell};

fn main() {
    let input = common::read_input!();
    let input = Boss::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

const SPELLS: [Spell; 5] = [
    Spell::new("MagicMissile", 53, 0, 0, 0, 4, 0),
    Spell::new("Drain", 73, 0, 0, 2, 2, 0),
    Spell::new("Shield", 113, 0, 7, 0, 0, 6),
    Spell::new("Poison", 173, 0, 0, 0, 3, 6),
    Spell::new("Recharge", 229, 101, 0, 0, 0, 5),
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Player {
    hit_points: i32,
    armor: i32,
    mana: i32,
    total_mana_spent: u32,
}
impl Player {
    fn new() -> Player {
        Player {
            hit_points: 50,
            armor: 0,
            total_mana_spent: 0,
            mana: 500,
        }
    }

    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GameState {
    player: Player,
    boss: Boss,
    active_spells: Vec<Spell>,
}

impl GameState {
    fn new(player: Player, boss: Boss) -> GameState {
        GameState {
            player,
            boss,
            active_spells: vec![],
        }
    }
    fn player_won(&self) -> bool {
        self.boss.is_dead() && !self.player.is_dead()
    }

    fn is_over(&self) -> bool {
        self.player.is_dead() || self.boss.is_dead()
    }

    fn apply_effects(&mut self) {
        if self.is_over() {
            return;
        }
        self.player.mana += self.active_spells.iter().map(|x| x.mana).sum::<i32>();
        self.player.armor = self.active_spells.iter().map(|x| x.armor).sum::<i32>();
        self.boss.hit_points -= self.active_spells.iter().map(|x| x.damage).sum::<i32>();

        self.active_spells = self
            .active_spells
            .iter()
            .filter_map(|spell| {
                (spell.turns > 1).then_some(Spell {
                    turns: spell.turns - 1,
                    ..*spell
                })
            })
            .collect();
    }

    fn available_spells(&self) -> Vec<Spell> {
        if self.is_over() {
            return vec![];
        }

        SPELLS
            .into_iter()
            .filter(|spell| {
                let already_active = self.active_spells.iter().any(|x| x.name == spell.name);
                let has_mana = spell.cost <= self.player.mana;

                !already_active && has_mana
            })
            .collect()
    }

    fn boss_turn(&mut self) {
        if self.is_over() {
            return;
        }
        // the boss always deals at least one damagae
        self.player.hit_points -= (self.boss.damage - self.player.armor).max(1);
    }

    fn player_turn(&mut self, spell: &Spell) {
        if self.is_over() {
            return;
        }
        // take away from our current mana and record the cost
        self.player.mana -= spell.cost;
        self.player.total_mana_spent += spell.cost as u32;

        // deal the damage or healing
        if spell.is_immediate() {
            self.player.hit_points += spell.healing;
            self.boss.hit_points -= spell.damage;
        } else {
            // this is an effect, so it goes in the active spells
            self.active_spells.push(spell.clone())
        }
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // flipped so that we can use a min-heap
        other
            .player
            .total_mana_spent
            .cmp(&self.player.total_mana_spent)
    }
}

fn simulate(game_state: GameState, hard_mode: bool) -> u32 {
    let mut best_so_far = u32::MAX;
    let mut priority_queue: BinaryHeap<GameState> = BinaryHeap::new();
    priority_queue.push(game_state);

    while let Some(mut game_state) = priority_queue.pop() {
        // if we've already spent too much mana compared to our best run, then bail
        if game_state.player.total_mana_spent >= best_so_far {
            continue;
        }

        // the player won, check if this is the best mana so far
        if game_state.player_won() {
            best_so_far = best_so_far.min(game_state.player.total_mana_spent);
            continue;
        }

        // hard mode decreases our hp by one every turn
        if hard_mode {
            game_state.player.hit_points -= 1;
        }

        // every round starts with applying effects
        game_state.apply_effects();

        // pick a spell to cast
        for spell in game_state.available_spells() {
            let mut next = game_state.clone();

            next.player_turn(&spell);
            next.apply_effects();
            next.boss_turn();

            // simulate the rest of the game based on the new state for this spell
            priority_queue.push(next);
        }
    }

    best_so_far
}

fn problem1(boss: &Boss) -> u32 {
    let player = Player::new();
    let game_state = GameState::new(player, *boss);

    simulate(game_state, false)
}

fn problem2(boss: &Boss) -> u32 {
    let player = Player::new();
    let game_state = GameState::new(player, *boss);

    simulate(game_state, true)
}

#[cfg(test)]
mod test {

    use crate::{problem1, problem2, Boss};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = Boss::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 900)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = Boss::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 1216)
    }
}
