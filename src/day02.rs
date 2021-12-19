#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(String, u8)> {
  input
    .lines()
    .map(|l| {
      let split = l.split_whitespace().collect::<Vec<&str>>();

      (split[0].to_string(), split[1].parse::<u8>().unwrap())
    })
    .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(String, u8)]) -> i32 {
  let mut depth: i32 = 0;
  let mut position: i32 = 0;
  for direction in input {
    match direction.0.as_str() {
      "forward" => position = position + i32::from(direction.1),
      "down" => depth = depth + i32::from(direction.1),
      "up" => depth = depth - i32::from(direction.1),
      _ => (println!("Ignored {}", direction.0)),
    }
  }
  println!("Final position: {}; Final Depth: {}", position, depth);
  return depth * position;
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(String, u8)]) -> i32 {
  let mut position: i32 = 0;
  let mut depth: i32 = 0;
  let mut aim: i32 = 0;
  for direction in input {
    match direction.0.as_str() {
      "forward" => {
        position = position + i32::from(direction.1);
        depth = depth + i32::from(direction.1) * aim;
      }
      "down" => aim = aim + i32::from(direction.1),
      "up" => aim = aim - i32::from(direction.1),
      _ => (println!("Ignored {}", direction.0)),
    }
  }
  println!("Final position: {}; Final Depth: {}", position, depth);
  return depth * position;
}
