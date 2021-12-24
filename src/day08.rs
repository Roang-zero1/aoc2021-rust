use std::collections::HashMap;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
  let mut lines = Vec::new();

  for line in input.lines() {
    let mut split_input = line.split("|");

    lines.push((
      split_input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| String::from(s))
        .collect(),
      split_input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| String::from(s))
        .collect(),
    ));
  }

  return lines;
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
  let mut counter: HashMap<usize, usize> = HashMap::new();
  for line in input {
    let line_clone = line.1.clone();
    for pattern in line_clone {
      let entry = counter.entry(pattern.len()).or_insert(0);
      *entry += 1;
    }
  }
  println!("Numbers counted: {:?}", counter);
  return counter.get(&(2 as usize)).unwrap()
    + counter.get(&(3 as usize)).unwrap()
    + counter.get(&(4 as usize)).unwrap()
    + counter.get(&(7 as usize)).unwrap();
}

pub fn get_chars(first: &Vec<char>, second: &Vec<char>) -> Vec<char> {
  first
    .clone()
    .into_iter()
    .filter(|item| !second.contains(item))
    .collect()
}

static ZERO: [char; 6] = ['a', 'b', 'c', 'e', 'f', 'g']; // 0
static ONE: [char; 2] = ['c', 'f']; // 1
static TWO: [char; 5] = ['a', 'c', 'd', 'e', 'g']; // 2
static THREE: [char; 5] = ['a', 'c', 'd', 'f', 'g']; // 3
static FOUR: [char; 4] = ['b', 'c', 'd', 'f']; // 4
static FIVE: [char; 5] = ['a', 'b', 'd', 'f', 'g']; // 5
static SIX: [char; 6] = ['a', 'b', 'd', 'e', 'f', 'g']; // 6
static SEVEN: [char; 3] = ['a', 'c', 'f']; // 7
static EIGHT: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g']; // 8
static NINE: [char; 6] = ['a', 'b', 'c', 'd', 'f', 'g']; //9

pub fn insert_number(
  number_table: &mut Vec<(Vec<char>, u8)>,
  number: u8,
  number_segments: Vec<char>,
  segment_table: &HashMap<char, char>,
) {
  let mut characters = Vec::new();
  for char_val in number_segments {
    characters.push(segment_table.get(&char_val).unwrap());
  }
  number_table.push((characters.iter().map(|c| **c).collect(), number));
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
  let mut output_sum = 0;
  for line in input {
    let mut patterns = line.0.clone();
    patterns.append(&mut line.1.clone());
    let mut counter: HashMap<char, usize> = HashMap::new();
    let mut segment_table: HashMap<char, char> = HashMap::new();
    let mut number_table: Vec<(Vec<char>, u8)> = Vec::new();
    let mut pattern_map: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
    for pattern in patterns {
      let entry = pattern_map.entry(pattern.len()).or_insert(Vec::new());
      let chars: Vec<char> = pattern.chars().collect();
      if entry
        .into_iter()
        .filter(|e| e.iter().all(|x| chars.contains(x)))
        .collect::<Vec<&mut Vec<char>>>()
        .len()
        == 0
      {
        entry.push(chars);
        for char_val in entry.last().unwrap() {
          let char_entry = counter.entry(*char_val).or_insert(0);
          *char_entry += 1;
        }
      }
    }
    segment_table.insert(
      'a',
      *get_chars(
        pattern_map.get(&(3 as usize)).unwrap().first().unwrap(),
        pattern_map.get(&(2 as usize)).unwrap().first().unwrap(),
      )
      .first()
      .unwrap(),
    );
    counter.remove(segment_table.get(&'a').unwrap());
    for (char_val, count) in counter.iter() {
      match count {
        4 => segment_table.insert('e', *char_val),
        6 => segment_table.insert('b', *char_val),
        8 => segment_table.insert('c', *char_val),
        9 => segment_table.insert('f', *char_val),
        _ => None,
      };
    }
    let mut rest_counter: HashMap<char, usize> = HashMap::new();
    for patter_six in pattern_map.get(&(6 as usize)).unwrap() {
      let rest = get_chars(patter_six, &(segment_table.values().map(|e| *e).collect()));
      for char_val in rest {
        let char_entry = rest_counter.entry(char_val).or_insert(0);
        *char_entry += 1;
      }
    }
    for (char_val, count) in rest_counter.iter() {
      match count {
        2 => segment_table.insert('d', *char_val),
        3 => segment_table.insert('g', *char_val),
        _ => None,
      };
    }

    insert_number(&mut number_table, 0, Vec::from(ZERO), &segment_table);
    insert_number(&mut number_table, 1, Vec::from(ONE), &segment_table);
    insert_number(&mut number_table, 2, Vec::from(TWO), &segment_table);
    insert_number(&mut number_table, 3, Vec::from(THREE), &segment_table);
    insert_number(&mut number_table, 4, Vec::from(FOUR), &segment_table);
    insert_number(&mut number_table, 5, Vec::from(FIVE), &segment_table);
    insert_number(&mut number_table, 6, Vec::from(SIX), &segment_table);
    insert_number(&mut number_table, 7, Vec::from(SEVEN), &segment_table);
    insert_number(&mut number_table, 8, Vec::from(EIGHT), &segment_table);
    insert_number(&mut number_table, 9, Vec::from(NINE), &segment_table);

    let output_digis = line.1.clone();

    let mut output_number: u32 = 0;

    for index in 0..output_digis.len() {
      let digit: Vec<char> = output_digis[index].chars().collect();
      for (number_segments, number) in &number_table {
        //println!("{:?}/{}",number_segments,number);
        if digit.len() == number_segments.len() {
          if digit.iter().all(|d| number_segments.contains(&d)) {
            output_number += (*number as u32) * (10 as u32).pow(3 - (index as u32));
            break;
          }
        }
      }
    }
    output_sum += output_number;
    println!("Found Output {}", output_number);
  }

  return output_sum;
}
