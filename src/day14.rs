use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (String, HashMap<(char, char), char>) {
  let mut lines = input.lines();
  let mut map: HashMap<(char, char), char> = HashMap::new();

  let start = String::from(lines.next().unwrap());
  lines.next();
  for line in lines {
    let mut chars = line.chars();
    map.insert(
      (chars.nth(0).unwrap(), chars.nth(0).unwrap()),
      chars.nth(4).unwrap(),
    );
  }
  return (start, map);
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &(String, HashMap<(char, char), char>)) -> usize {
  let mut output: Vec<char> = input.0.chars().collect();
  let map = &input.1;

  for _ in 0..10 {
    let tmp = output.clone();
    let mut origin = tmp.iter();
    output = Vec::new();
    let mut last: char = *origin.next().unwrap();
    output.push(last);

    for next in origin {
      output.push(*map.get(&(last, *next)).unwrap());
      output.push(*next);
      last = *next;
    }
  }

  let mut counter: HashMap<char, usize> = HashMap::new();
  for x in output {
    *counter.entry(x).or_default() += 1;
  }

  let max = counter.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
  let min = counter.iter().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

  println!(
    "Found {} chars, with {} at maxium count of {} and {} at minimum count of {}",
    counter.values().sum::<usize>(),
    max.0,
    max.1,
    min.0,
    min.1
  );

  return max.1 - min.1;
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &(String, HashMap<(char, char), char>)) -> usize {
  let mut mutation_map: HashMap<(char, char), ((char, char), (char, char))> = HashMap::new();

  for mutation in &input.1 {
    mutation_map.insert(
      *mutation.0,
      ((mutation.0 .0, *mutation.1), (*mutation.1, mutation.0 .1)),
    );
  }

  let mut pairs: HashMap<(char, char), usize> = HashMap::new();

  let mut input_chars = input.0.chars();
  let mut last = input_chars.next().unwrap();

  for _ in 0..input.0.len() - 1 {
    let next = input_chars.next().unwrap();
    *pairs.entry((last, next)).or_default() += 1;
    last = next;
  }

  let mut counter: HashMap<char, usize> = HashMap::new();

  for input_char in input.0.chars() {
    *counter.entry(input_char).or_default() += 1;
  }

  for _ in 0..40 {
    for (pair, count) in pairs.clone().iter() {
      if *count == 0 {
        continue;
      }
      let mutation = mutation_map.get(pair).unwrap();
      *counter.entry(mutation.0 .1).or_default() += *count;
      *pairs.entry(*pair).or_default() -= *count;
      *pairs.entry(mutation.0).or_default() += *count;
      *pairs.entry(mutation.1).or_default() += *count;
    }
  }

  let max = counter.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
  let min = counter.iter().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

  println!(
    "Found {} chars, with {} at maxium count of {} and {} at minimum count of {}",
    counter.values().sum::<usize>(),
    max.0,
    max.1,
    min.0,
    min.1
  );

  return max.1 - min.1;
}
