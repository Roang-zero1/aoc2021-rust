#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<u32> {
  input
    .split(",")
    .map(|l| l.parse::<u32>().unwrap())
    .collect()
}

pub fn fuel_calculator(input: &[u32], calculator: fn(isize, isize) -> isize) -> isize {
  let min = *input.iter().min().unwrap();
  let max = *input.iter().max().unwrap();

  let mut min_fuel: Option<isize> = None;

  for position in (min as usize)..((max + 1) as usize) {
    let mut fuel = 0;
    for crab in input {
      fuel += calculator(*crab as isize, position as isize);
      if min_fuel != None && Some(fuel) > min_fuel {
        break;
      }
    }
    if min_fuel == None || Some(fuel) < min_fuel {
      min_fuel = Some(fuel)
    }
  }

  println!("{:?}: {}/{}: {}", input.len(), min, max, min_fuel.unwrap());
  return min_fuel.unwrap();
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[u32]) -> isize {
  return fuel_calculator(input, |crab, position| (crab - position).abs());
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[u32]) -> isize {
  return fuel_calculator(input, |crab, position| {
    let differnece = (crab - position).abs();
    (1..differnece + 1).sum()
  });
}
