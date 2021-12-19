#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
  input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(_input: &[u32]) -> u32 {
}

#[aoc(day1, part2)]
pub fn solve_part2(_input: &[u32]) -> u32 {
  return 0
}
