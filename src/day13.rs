use std::collections::HashMap;
use std::fmt;

pub struct FoldInstruction {
  horizontal: bool,
  fold_line: u16,
}

impl fmt::Display for FoldInstruction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.horizontal, self.fold_line)
  }
}

impl fmt::Debug for FoldInstruction {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt
      .debug_struct("FoldInstruction")
      .field("horizontal", &self.horizontal)
      .field("fold_line", &self.fold_line)
      .finish()
  }
}

#[derive(Clone, Copy)]
pub struct Coordinate {
  x: u16,
  y: u16,
}

impl Coordinate {
  fn fold(&mut self, fold_instruction: &FoldInstruction) {
    if fold_instruction.horizontal {
      if self.x > fold_instruction.fold_line {
        self.x = 2 * fold_instruction.fold_line - self.x
      }
    } else {
      if self.y > fold_instruction.fold_line {
        self.y = 2 * fold_instruction.fold_line - self.y
      }
    }
  }
}

impl fmt::Display for Coordinate {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl fmt::Debug for Coordinate {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt
      .debug_struct("Coordinate")
      .field("x", &self.x)
      .field("y", &self.y)
      .finish()
  }
}

fn count_coordinates(points: &Vec<Coordinate>, print: Option<bool>) -> u16 {
  let output = print.unwrap_or(true);
  let height = points.iter().max_by_key(|p| p.y).unwrap().y + 1;
  let width = points.iter().max_by_key(|p| p.x).unwrap().x + 1;
  let mut map: HashMap<u16, Vec<u16>> = HashMap::new();
  let mut count = 0;

  if output {
    println!("-------------");
  }

  for point in points {
    let entry = map.entry(point.y).or_insert(Vec::new());
    entry.push(point.x);
  }

  for y in 0..height {
    for x in 0..width {
      let entry = map.entry(y).or_insert(Vec::new());
      if output {
        if entry.contains(&x) {
          print!("#");
          count += 1
        } else {
          print!(".")
        }
      }
    }
    if output {
      println!();
    }
  }
  return count;
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (Vec<Coordinate>, Vec<FoldInstruction>) {
  let mut point = true;
  let mut points: Vec<Coordinate> = Vec::new();
  let mut folds: Vec<FoldInstruction> = Vec::new();
  for line in input.lines() {
    if line.len() == 0 {
      point = false;
      continue;
    }
    if point {
      let coordinates = line
        .split(",")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
      points.push(Coordinate {
        x: coordinates[0].parse::<u16>().unwrap(),
        y: coordinates[1].parse::<u16>().unwrap(),
      })
    } else {
      folds.push(FoldInstruction {
        horizontal: line.chars().nth(11).unwrap() == 'x',
        fold_line: String::from(line.split_at(13).1).parse::<u16>().unwrap(),
      });
    }
  }
  println!("{:?}", folds);
  return (points, folds);
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(Vec<Coordinate>, Vec<FoldInstruction>)) -> u16 {
  let mut points = input.0.clone();
  let inst = input.1.iter().next().unwrap();
  for point in points.iter_mut() {
    point.fold(inst);
  }
  return count_coordinates(&points, Some(false));
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(Vec<Coordinate>, Vec<FoldInstruction>)) -> u16 {
  let mut points = input.0.clone();
  for instruction in &input.1 {
    for point in points.iter_mut() {
      point.fold(instruction);
    }
  }
  count_coordinates(&points, Some(true));

  return 0
}
