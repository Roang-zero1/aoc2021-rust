use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Coordinate {
  x: i8,
  y: i8,
}

impl Coordinate {
  fn valid(&self) -> bool {
    self.x >= 0 && self.x < 10 && self.y >= 0 && self.y < 10
  }
}
impl PartialEq for Coordinate {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}
impl Eq for Coordinate {}

impl Hash for Coordinate {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.hash(state);
    self.y.hash(state);
  }
}

impl fmt::Display for Coordinate {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> HashMap<Coordinate, u8> {
  let mut map: HashMap<Coordinate, u8> = HashMap::new();

  let mut lines = input.lines();

  for y in 0..10 {
    let values = lines
      .next()
      .unwrap()
      .chars()
      .map(|c| String::from(c).parse::<u8>().unwrap())
      .collect::<Vec<u8>>();
    for x in 0..values.len() {
      map.insert(
        Coordinate {
          x: x as i8,
          y: y as i8,
        },
        values[x],
      );
    }
  }
  return map;
}

pub fn print_map(map: &HashMap<Coordinate, u8>) {
  println!("-----------");
  for y in 0..10 {
    for x in 0..10 {
      print!("{}", map.get(&Coordinate { x, y }).unwrap());
    }
    println!();
  }
}

fn increase(map: &mut HashMap<Coordinate, u8>, c: &Coordinate) -> u32 {
  let mut flashes = 0;
  if !c.valid() {
    return 0;
  }
  let current = map.entry(*c).or_default();
  *current += 1;
  if *current == 10 {
    flashes += 1;
    for y in c.y - 1..c.y + 2 {
      for x in c.x - 1..c.x + 2 {
        flashes += increase(map, &Coordinate { x, y });
      }
    }
  }
  return flashes;
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &HashMap<Coordinate, u8>) -> u32 {
  let mut map = input.clone();
  let mut flashes = 0;
  for _ in 0..100 {
    for y in 0..10 {
      for x in 0..10 {
        flashes += increase(&mut map, &Coordinate { x, y });
      }
    }

    for y in 0..10 {
      for x in 0..10 {
        let entry = map.entry(Coordinate { x, y }).or_default();
        if *entry > 9 {
          *entry = 0;
        }
      }
    }
  }
  return flashes;
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &HashMap<Coordinate, u8>) -> u32 {
  let mut map = input.clone();
  let mut step = 0;
  let mut done = false;
  while !done {
    for y in 0..10 {
      for x in 0..10 {
        increase(&mut map, &Coordinate { x, y });
      }
    }

    let mut flashes = 0;
    for y in 0..10 {
      for x in 0..10 {
        let entry = map.entry(Coordinate { x, y }).or_default();
        if *entry > 9 {
          *entry = 0;
          flashes += 1;
        }
      }
    }
    if flashes == 100 {
      done = true;
    }
    step += 1;
  }
  return step;
}
