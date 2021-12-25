use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|l| l.chars().collect()).collect()
}

fn analyze_line(
  line: &Vec<char>,
  ascii_map: &HashMap<char, char>,
) -> (Option<char>, Option<Vec<char>>) {
  let mut openings: Vec<char> = Vec::new();
  let mut missing_closings = Vec::new();
  for char_val in line {
    match char_val {
      '(' | '[' | '{' | '<' => openings.push(*char_val),
      ')' | ']' | '}' | '>' => {
        let opening = openings.pop().unwrap();
        if ascii_map.get(&opening).unwrap() != char_val {
          return (Some(*char_val), None);
        }
      }
      _ => (),
    }
  }
  for opening in openings.iter().rev() {
    missing_closings.push(*ascii_map.get(&opening).unwrap())
  }
  return (None, Some(missing_closings));
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> u32 {
  let ascii_map: HashMap<char, char> =
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
  let mut counter: HashMap<char, u8> = HashMap::new();
  for line in input {
    let (error, _missing_closings) = analyze_line(line, &ascii_map);
    if error != None {
      let entry = counter.entry(error.unwrap()).or_insert(0);
      *entry += 1;
    }
  }

  println!("Found the following errors: {:?}", counter);
  let mut points: u32 = 0;
  for (char_val, count) in counter.iter() {
    match char_val {
      ')' => points += *count as u32 * 3,
      ']' => points += *count as u32 * 57,
      '}' => points += *count as u32 * 1197,
      '>' => points += *count as u32 * 25137,
      _ => (),
    }
  }
  return points;
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> u64 {
  let ascii_map: HashMap<char, char> =
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
  let mut scores: Vec<u64> = Vec::new();
  let mut counter = 0;
  for line in input {
    let (_error, missing_closings) = analyze_line(line, &ascii_map);
    if missing_closings != None {
      let mut score: u64 = 0;
      for closing in missing_closings.unwrap() {
        counter += 1;
        score *= 5;
        match closing {
          ')' => score += 1,
          ']' => score += 2,
          '}' => score += 3,
          '>' => score += 4,
          _ => (),
        }
      }
      scores.push(score);
    }
  }

  scores.sort();

  println!(
    "Found {} missing closings and {} scores",
    counter,
    scores.len()
  );
  return scores[scores.len() / 2];
}
