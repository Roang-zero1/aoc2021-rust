use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Clone)]
struct Coordinates {
  row: usize,
  line: usize,
}

#[allow(dead_code)]
#[derive(Clone)]
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

  for _number in 0..100 {
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
  let mut boards = input.2.clone();

  let mut winner: Option<(u8, usize)> = None;

  'outer: for number in input.0.iter() {
    for board_index in input.1[*number as usize].iter() {
      let mut board = boards[*board_index as usize].clone();
      let coordinates = board.numbers.remove(number).unwrap();
      board.rows[coordinates.row] += 1;
      if board.rows[coordinates.row] >= 5 {
        winner = Some((*number, *board_index as usize));
        break 'outer;
      }
      board.lines[coordinates.line] += 1;
      if board.lines[coordinates.line] >= 5 {
        winner = Some((*number, *board_index as usize));
        break 'outer;
      }
      boards[*board_index as usize] = board;
    }
  }

  if let Some(winner_values) = winner {
    println!(
      "Board {} won with number {}",
      winner_values.1, winner_values.0
    );
    let unmarked_sum = boards[winner_values.1]
      .numbers
      .keys()
      .map(|n| *n as u32)
      .sum::<u32>()
      - winner_values.0 as u32;
    return unmarked_sum * winner_values.0 as u32;
  } else {
    panic!("No winner detected");
  }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<u8>, Vec<Vec<u16>>, Vec<Board>)) -> u32 {
  let mut boards = input.2.clone();
  let mut winning_boards: HashSet<u16> = HashSet::new();

  let mut winner: Option<(u8, usize)> = None;

  let number_of_boards = boards.len();

  'outer: for number in input.0.iter() {
    for board_index in input.1[*number as usize].iter() {
      if winning_boards.contains(board_index) {
        continue;
      }
      let mut board = boards[*board_index as usize].clone();
      let coordinates = board.numbers.remove(number).unwrap();
      board.rows[coordinates.row] += 1;
      if board.rows[coordinates.row] >= 5 {
        winning_boards.insert(*board_index);
        if winning_boards.len() == number_of_boards {
          winner = Some((*number, *board_index as usize));
          break 'outer;
        }
        continue;
      }
      board.lines[coordinates.line] += 1;
      if board.lines[coordinates.line] >= 5 {
        winning_boards.insert(*board_index);
        if winning_boards.len() == number_of_boards {
          winner = Some((*number, *board_index as usize));
          break 'outer;
        }
        continue;
      }
      boards[*board_index as usize] = board;
    }
  }

  if let Some(winner_values) = winner {
    println!(
      "Board {} won with number {}",
      winner_values.1, winner_values.0
    );
    let unmarked_sum = boards[winner_values.1]
      .numbers
      .keys()
      .map(|n| *n as u32)
      .sum::<u32>()
      - winner_values.0 as u32;
    return unmarked_sum * winner_values.0 as u32;
  } else {
    panic!("No winner detected");
  }
}
