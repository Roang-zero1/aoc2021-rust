#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> [Vec<u8>; 12] {
  let mut values: [Vec<u8>; 12] = Default::default();
  for line in input.lines() {
    let chars = line.chars().collect::<Vec<char>>();
    for x in 0..12 {
      values[x].push(if chars[x] == '1' { 1 } else { 0 });
    }
  }
  return values;
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<u8>; 12]) -> u32 {
  let max = 12;
  let mut gamma_rate = 0;
  let mut epsilon_rate = 0;
  for index in 0..max {
    let ones = input[index].iter().filter(|&n| *n == 1).count();
    let zeros = input[index].iter().filter(|&n| *n == 0).count();
    println!(
      "{}/{}, {} pow {}",
      ones,
      zeros,
      ones + zeros,
      4 - index as u32
    );
    if ones > zeros {
      gamma_rate = gamma_rate + 2u32.pow(max as u32 - 1 - index as u32);
    } else {
      epsilon_rate = epsilon_rate + 2u32.pow(max as u32 - 1 - index as u32);
    }
  }
  println!("Gamma rate: {}; Epsilon rate: {}", gamma_rate, epsilon_rate);
  return gamma_rate * epsilon_rate;
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<u8>; 12]) -> u32 {
  return 0;
}
