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

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<Line>) -> usize {
  let mut map: HashMap<(usize, usize), u8> = HashMap::new();
  for line in input {
    if line.start.x == line.end.x {
      let y_values = [line.start.y, line.end.y];
      let min = *y_values.iter().min().unwrap();
      let max = *y_values.iter().max().unwrap();
      for y in min..max + 1 {
        let count = map.entry((line.start.x, y)).or_insert(0);
        *count += 1;
      }
    }
    if line.start.y == line.end.y {
      let x_values = [line.start.x, line.end.x];
      let min = *x_values.iter().min().unwrap();
      let max = *x_values.iter().max().unwrap();
      for x in min..max + 1 {
        let count = map.entry((x, line.start.y)).or_insert(0);
        *count += 1;
      }
    }
  }
  return map.values().filter(|n| *n >= &(2 as u8)).count();
}

#[aoc(day5, part2)]
pub fn solve_part2(_input: &Vec<Line>) -> u32 {
  return 0;
}
