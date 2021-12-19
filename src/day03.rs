pub fn bit_counter(input: &Vec<u16>, power: u8) -> u8 {
  let ones = input
    .iter()
    .map(|number| if number & (1 << power) != 0 { 1 } else { 0 })
    .filter(|n| *n == 1)
    .count();
  if ones as f32 >= input.len() as f32 / 2.0 {
    return 1;
  }
  return 0;
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<u16>, usize) {
  let mut max = 0;
  let input = input
    .lines()
    .map(|l| {
      max = l.len();
      u16::from_str_radix(l, 2).unwrap()
    })
    .collect();
  return (input, max);
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &(Vec<u16>, usize)) -> u32 {
  let mut gamma_rate = 0;
  let mut epsilon_rate = 0;
  for power in 0..input.1 {
    if bit_counter(&input.0, power as u8) == 1 {
      gamma_rate = gamma_rate + 2u32.pow(power as u32);
    } else {
      epsilon_rate = epsilon_rate + 2u32.pow(power as u32);
    }
  }
  println!("Gamma rate: {}; Epsilon rate: {}", gamma_rate, epsilon_rate);
  return gamma_rate * epsilon_rate;
}

pub fn value_filter(input: &(Vec<u16>, usize), favour_most_common: bool) -> u32 {
  let mut oxygen_vector = input.0.to_vec();
  for power in (0..input.1).rev() {
    let most_common = bit_counter(&oxygen_vector, power as u8);

    oxygen_vector = oxygen_vector
      .iter()
      .filter(|number| {
        (if (*number & (1 << power) != 0) ^ !favour_most_common {
          1
        } else {
          0
        }) == most_common
      })
      .cloned()
      .collect();
    if oxygen_vector.len() == 1 {
      break;
    }
  }

  return oxygen_vector[0] as u32;
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &(Vec<u16>, usize)) -> u32 {
  let oxygen_generator_rating = value_filter(input, true);
  println!("--------------------------------");
  let co2_scrubber_rating = value_filter(input, false);

  println!(
    "oxygen generator rating: {}; CO2 scrubber rating: {}",
    oxygen_generator_rating, co2_scrubber_rating
  );
  return oxygen_generator_rating * co2_scrubber_rating;
}
