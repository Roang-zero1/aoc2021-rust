use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (String, HashMap<(char,char), char>) {
  let mut lines = input.lines();
  let mut map: HashMap<(char,char), char> = HashMap::new();

  let start = String::from(lines.next().unwrap());
  lines.next();
  for line in lines {
    let mut chars = line.chars();
    map.insert((chars.nth(0).unwrap(),chars.nth(0).unwrap(),),chars.nth(4).unwrap());
  }
  return (start, map);
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &(String, HashMap<(char,char), char>)) -> usize {
  let mut output: Vec<char> = input.0.chars().collect();
  let map = &input.1;

  for _ in 0..10 {
    let tmp = output.clone();
    let mut origin = tmp.iter();
    output = Vec::new();
    let mut last: char = *origin.next().unwrap();
    output.push(last);

    for next in origin {
      output.push(*map.get(&(last,*next)).unwrap());
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

fn enhance_polymer(
  polymer: Vec<char>,
  enhancement_map: &HashMap<(char,char), char>,
  depth: usize,
  max_depth: usize,
  counter: &mut HashMap<char, usize>,
) {

  let polymer_insert = *enhancement_map
    .get(&(polymer[0],polymer[1]))
    .unwrap();

  *counter.entry(polymer_insert).or_default() += 1;

  if depth == 0 {
    println!(
      "D: {} -> Enhancing Polymer {} inserting {}",
      depth, String::from_iter(polymer.clone()), polymer_insert
    );
  }

  if depth < max_depth - 1 {
    enhance_polymer(
      Vec::from([polymer[0],polymer_insert]),
      enhancement_map,
      depth + 1,
      max_depth,
      counter,
    );
    enhance_polymer(
      Vec::from([polymer_insert,polymer[1]]),
      enhancement_map,
      depth + 1,
      max_depth,
      counter,
    );
  }
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &(String, HashMap<(char,char), char>)) -> usize {
  let mut counter: HashMap<char, usize> = HashMap::new();
  let max_depth = 40;

  for idx in 0..input.0.len() - 1 {
    *counter
      .entry(input.0.chars().nth(idx).unwrap())
      .or_default() += 1;
    enhance_polymer(
      String::from(&input.0[idx..idx + 2]).chars().collect(),
      &input.1,
      0,
      max_depth,
      &mut counter
    );
  }

  *counter
    .entry(input.0.chars().nth(input.0.len() - 1).unwrap())
    .or_default() += 1;

  println!("{:?}", counter);

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
