#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
  input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
  let mut increases: u32 = 0;
  let mut last: u32 = input[0];
  for x in 1..input.len() {
    if input[x] > last {
      increases = increases + 1;
    }
    last = input[x]
  }

  return increases;
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
  let mut increases: u32 = 0;
  let mut last: u32 = input[0] + input[1] + input[2];
  for x in 1..input.len() - 2 {
    let value = input[x] + input[x + 1] + input[x + 2];
    if value > last {
      increases = increases + 1;
    }
    last = value
  }

  return increases;
}
