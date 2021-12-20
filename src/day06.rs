use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> HashMap<u8, usize> {
  let fish_raw = input.split(",").map(|l| l.parse::<u8>().unwrap());
  let mut fishes: HashMap<u8, usize> = HashMap::new();
  for age in 0..6 {
    let count = fish_raw.clone().filter(|f| *f == age as u8).count();
    fishes.insert(age, count);
  }
  return fishes;
}

pub fn print_pop(fishes: &HashMap<u8, usize>){
  print!("Groups: ");
  for age in 0..9{
    print!("{}: {:4}, ", age, fishes.get(&(age as u8)).unwrap_or(&(0 as usize)));
  }
  println!()
}

pub fn grow_pop(fishes: HashMap<u8, usize>) -> HashMap<u8, usize> {
  let mut new_fishes: HashMap<u8, usize> = HashMap::new();
  let save_values = [1, 2, 3, 4, 5, 6, 8];
  for age in save_values {
    let count = *fishes.get(&(age as u8)).unwrap_or(&(0 as usize));
    new_fishes.insert(age - 1, count);
  }
  let zero_count = *fishes.get(&(0 as u8)).unwrap_or(&(0 as usize));
  new_fishes.insert(6, zero_count + fishes.get(&(7 as u8)).unwrap_or(&(0 as usize)));
  new_fishes.insert(8, zero_count);

  return new_fishes;
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &HashMap<u8, usize>) -> usize {
  let mut fishes = input.clone();
  for _ in 0..80 {
    fishes = grow_pop(fishes);
  }
  println!("Final makeup of population:");
  print_pop(&fishes);
  return fishes.values().sum();
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &HashMap<u8, usize>) -> usize {
  let mut fishes = input.clone();
  for _ in 0..256 {
    fishes = grow_pop(fishes);
  }
  println!("Final makeup of population:");
  print_pop(&fishes);
  return fishes.values().sum();
}
