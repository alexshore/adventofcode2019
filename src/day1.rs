use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    masses: Vec<u32>
}

impl FromIterator<u32> for Input {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut masses = vec![];

        for i in iter {
            masses.push(i);
        }

        Self { masses }
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Input {
    input.lines().map(
        |line| line.parse().expect("oops")
    ).collect()
}

fn calc_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

#[aoc(day1, part1)]
fn part_one(input: &Input) -> u32 {
    input.masses.iter().fold(0, |acc, mass| acc + calc_fuel(*mass))
}

#[aoc(day1, part2)]
fn part_two(input: &Input) -> u32 {
    input.masses.iter().fold(0, |acc, mass| {
        let mut last_mass = calc_fuel(*mass);
        let mut res = last_mass;
        while last_mass > 2 {
            last_mass = calc_fuel(last_mass);
            res += last_mass;
        }

        acc + res
    })
}