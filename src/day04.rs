use std::collections::HashMap;

#[allow(dead_code)]
struct Coordinates {
  row: u8,
  line: u8,
}

#[allow(dead_code)]
pub struct Board {
  numbers: HashMap<u8, Coordinates>,
  rows: [u8; 5],
  lines: [u8; 5],
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u8>, Vec<Vec<u16>>, Vec<Board>) {
  let mut raw_input = input.split("\n\n");

  let draw_numbers = raw_input
    .next()
    .unwrap()
    .split(",")
    .map(|raw_number| raw_number.parse::<u8>().unwrap())
    .collect::<Vec<u8>>();

  let mut numbers_on_boards: Vec<Vec<u16>> = Vec::new();
  let mut boards: Vec<Board> = Vec::new();

  for number in 0..100 {
    numbers_on_boards.push(Vec::new())
  }

  let mut board_number = 0;

  for board_str in raw_input {
    let mut board = Board {
      numbers: HashMap::new(),
      rows: [0, 0, 0, 0, 0],
      lines: [0, 0, 0, 0, 0],
    };
    let mut line_number = 0;
    for line_str in board_str.lines() {
      let mut row_number = 0;
      for number_str in line_str.split_whitespace() {
        let number = number_str.parse::<u8>().unwrap();
        numbers_on_boards[number as usize].push(board_number);
        board.numbers.insert(
          number,
          Coordinates {
            row: row_number,
            line: line_number,
          },
        );
        row_number += 1;
      }
      line_number += 1;
    }
    boards.push(board);
    board_number += 1
  }
  println!("Boards: {}", boards.len());
  return (draw_numbers, numbers_on_boards, boards);
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(Vec<u8>, Vec<Vec<u16>>, Vec<Board>)) -> u32 {
  return 0;
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<u8>, Vec<Vec<u16>>, Vec<Board>)) -> u32 {
  return 0;
}
