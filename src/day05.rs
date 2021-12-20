use std::collections::HashMap;

struct Coordinates {
  x: usize,
  y: usize,
}

pub struct Line {
  start: Coordinates,
  end: Coordinates,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
  let mut lines: Vec<Line> = Vec::new();
  for line in input.lines() {
    let mut coordinates = line.split(" -> ");
    let mut start = coordinates.next().unwrap().split(',');
    let mut end = coordinates.next().unwrap().split(',');

    lines.push(Line {
      start: Coordinates {
        x: start.next().unwrap().parse::<usize>().unwrap(),
        y: start.next().unwrap().parse::<usize>().unwrap(),
      },
      end: Coordinates {
        x: end.next().unwrap().parse::<usize>().unwrap(),
        y: end.next().unwrap().parse::<usize>().unwrap(),
      },
    });
  }

  return lines;
}

pub fn map_vents(input: Vec<&Line>) -> HashMap<(usize, usize), u8> {
  let mut map: HashMap<(usize, usize), u8> = HashMap::new();
  for line in input {
    let x_diff = line.end.x as isize - line.start.x as isize;
    let y_diff = line.end.y as isize - line.start.y as isize;

    let mut x_pos = line.start.x;
    let mut y_pos = line.start.y;

    for _ in 0..(if x_diff != 0 { x_diff.abs() + 1 } else { y_diff.abs() + 1 }) {
      let count = map.entry((x_pos, y_pos)).or_insert(0);
      *count += 1;

      if x_diff > 0 {
        x_pos += 1;
      } else if x_diff < 0 {
        x_pos -= 1;
      }

      if y_diff > 0 {
        y_pos += 1;
      } else if y_diff < 0 {
        y_pos -= 1;
      }
    }
  }

  return map;
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<Line>) -> usize {
  let lines: Vec<&Line> = input
    .iter()
    .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
    .collect();
  let map = map_vents(lines);
  return map.values().filter(|n| *n >= &(2 as u8)).count();
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<Line>) -> usize {
  let map = map_vents(input.iter().collect());
  return map.values().filter(|n| *n >= &(2 as u8)).count();
}
