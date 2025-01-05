use aoc_runner_derive::{aoc, aoc_generator};

struct Input(Vec<usize>);

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    Input(input.split(',').map(|i| i.parse::<usize>().expect("oops")).collect())
}

#[derive(Debug)]
struct OpCode(usize, usize, usize, usize);

impl OpCode {
    fn new(lst: &[usize]) -> Self {
        Self(lst[0], lst[1], lst[2], lst[3])
    }
}

struct IntCoder {
    pc: usize,
    memory: Vec<usize>,
}

impl IntCoder {
    fn new(memory: Vec<usize>) -> Self {
        Self {
            pc: 0,
            memory
        }
    }

    fn run(&mut self) {
        loop {
            if self.memory[self.pc] == 99 {
                break
            }

            let intcode = OpCode::new(&self.memory[self.pc..self.pc + 4]);

            match intcode.0 {
                1 => self.memory[intcode.3] = self.memory[intcode.1] + self.memory[intcode.2],
                2 => self.memory[intcode.3] = self.memory[intcode.1] * self.memory[intcode.2],
                _ => unreachable!()
            }

            self.pc += 4
        }
    } 
}

#[aoc(day2, part1)]
fn part_one(input: &Input) -> usize {
    let mut intcoder = IntCoder::new(input.0.clone());

    intcoder.memory[1] = 12;
    intcoder.memory[2] = 2;

    intcoder.run();

    intcoder.memory[0]
}

#[aoc(day2, part2)]
fn part_two(input: &Input) -> usize {
    let mut intcoder = IntCoder::new(input.0.clone());

    for noun in 0..100 {
        for verb in 0..100 {
            intcoder.memory = input.0.clone();
            intcoder.pc = 0;

            intcoder.memory[1] = noun;
            intcoder.memory[2] = verb;

            intcoder.run();
            
            if intcoder.memory[0] == 19690720 {
                return noun * 100 + verb
            }
        }
    }

    unreachable!()
}