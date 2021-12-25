use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
  let mut lines: Vec<Vec<u8>> = Vec::new();
  for line in input.lines() {
    lines.push(
      line
        .chars()
        .map(|c| String::from(c).parse::<u8>().unwrap())
        .collect::<Vec<u8>>(),
    )
  }
  return lines;
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Vec<u8>>) -> u32 {
  let max_height = input.len();
  let max_width = input[0].len();

  let mut risk_level_sum: u32 = 0;
  let mut count: u16 = 0;

  for y in 0..max_height {
    for x in 0..max_width {
      let mut lower = 0;
      let current = input[y][x];
      if y != 0 {
        if input[y - 1][x] > current {
          lower += 1;
        }
      } else {
        lower += 1;
      }
      if y + 1 < max_height {
        if input[y + 1][x] > current {
          lower += 1;
        }
      } else {
        lower += 1;
      }
      if x != 0 {
        if input[y][x - 1] > current {
          lower += 1;
        }
      } else {
        lower += 1;
      }
      if x + 1 < max_width {
        if input[y][x + 1] > current {
          lower += 1;
        }
      } else {
        lower += 1;
      }

      if lower == 4 {
        risk_level_sum += (current + 1) as u32;
        count += 1;
      }
    }
  }

  println!(
    "Found {} low points with a total risk rating of {}",
    count, risk_level_sum
  );

  return risk_level_sum;
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Vec<u8>>) -> u32 {
  let mut visited: HashSet<(u8, u8)> = HashSet::new();
  let max_height = input.len();
  let max_width = input[0].len();

  fn check_point(
    input: &Vec<Vec<u8>>,
    visited: &mut HashSet<(u8, u8)>,
    max_width: usize,
    max_height: usize,
    x: usize,
    y: usize,
  ) -> u16 {
    let mut sum: u16 = 0;
    if visited.contains(&(x as u8, y as u8)) {
      return 0;
    }
    let current = input[y][x];
    if current == 9 {
      return 0;
    }
    visited.insert((x as u8, y as u8));
    sum += 1;
    if x + 1 < max_width {
      sum += check_point(input, visited, max_width, max_height, x + 1, y);
    }
    if y + 1 < max_height {
      sum += check_point(input, visited, max_width, max_height, x, y + 1);
    }
    if x > 0 {
      sum += check_point(input, visited, max_width, max_height, x - 1, y);
    }
    if y > 0 {
      sum += check_point(input, visited, max_width, max_height, x, y - 1);
    }
    return sum;
  }
  let mut areas: Vec<u16> = Vec::new();
  for y in 0..max_height {
    for x in 0..max_width {
      let area = check_point(input, &mut visited, max_width, max_height, x, y);
      if area > 0 {
        areas.push(area);
      }
    }
  }

  areas.sort_by(|a,b,| a.cmp(b).reverse());
  areas.truncate(3);

  println!("Three largest areas are: {:?}",areas);

  let mut result = 1;

  for area in areas {
    result *= area as u32;
  }

  return result;
}
