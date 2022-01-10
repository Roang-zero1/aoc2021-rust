use std::num::ParseIntError;
use std::vec::Drain;

trait FromStrRadix<T> {
  fn from_str_radix(src: &str, radix: u32) -> Result<T, ParseIntError>;
}

impl FromStrRadix<usize> for usize {
  fn from_str_radix(src: &str, radix: u32) -> Result<usize, ParseIntError> {
    usize::from_str_radix(src, radix)
  }
}

impl FromStrRadix<u8> for u8 {
  fn from_str_radix(src: &str, radix: u32) -> Result<u8, ParseIntError> {
    u8::from_str_radix(src, radix)
  }
}

impl FromStrRadix<u16> for u16 {
  fn from_str_radix(src: &str, radix: u32) -> Result<u16, ParseIntError> {
    u16::from_str_radix(src, radix)
  }
}

impl FromStrRadix<u64> for u64 {
  fn from_str_radix(src: &str, radix: u32) -> Result<u64, ParseIntError> {
    u64::from_str_radix(src, radix)
  }
}

#[derive(Debug)]
pub struct Packet {
  version: u8,
  value: u64,
  sub_packets: Option<Vec<Packet>>,
}

fn number_from_drain<T: FromStrRadix<T>>(drain: &mut Drain<char>) -> T {
  T::from_str_radix(&drain.collect::<String>(), 2).unwrap()
}

fn number_from_vec<T: FromStrRadix<T>>(vec: &Vec<char>) -> T {
  T::from_str_radix(&vec.into_iter().collect::<String>(), 2).unwrap()
}

fn parse_next_packet(bit_vec: &mut Vec<char>) -> Result<(Packet, usize), &'static str> {
  let mut packet_length = 6;
  let version = number_from_drain::<u8>(&mut bit_vec.drain(0..3));
  let p_type = number_from_drain::<u8>(&mut bit_vec.drain(0..3));

  match p_type {
    4 => {
      let mut data_vec: Vec<char> = Vec::new();
      while {
        packet_length += 5;
        let mut bits = bit_vec.drain(0..5);
        let next = bits.next().unwrap();
        data_vec.append(&mut bits.collect::<Vec<char>>());
        next == '1'
      } {}
      return Ok((
        Packet {
          version: version,
          value: number_from_vec::<u64>(&data_vec),
          sub_packets: None,
        },
        packet_length,
      ));
    }
    _ => {
      let length_type = number_from_drain::<u8>(&mut bit_vec.drain(0..1));
      packet_length += 1;
      let mut packets: Vec<Packet> = Vec::new();
      let mut values: Vec<u64> = Vec::new();
      match length_type {
        0 => {
          let mut remaining_length = number_from_drain::<usize>(&mut bit_vec.drain(0..15));
          packet_length += 15;
          while remaining_length > 0 {
            let result = parse_next_packet(bit_vec);
            match result {
              Ok(v) => {
                let (packet, inner_length) = v;
                values.push(packet.value);
                packets.push(packet);
                remaining_length -= inner_length;
                packet_length += inner_length
              }
              Err(e) => return Err(e),
            }
          }
        }
        1 => {
          let number_of_packets = number_from_drain::<usize>(&mut bit_vec.drain(0..11));
          packet_length += 11;
          for _ in 0..number_of_packets {
            let result = parse_next_packet(bit_vec);
            match result {
              Ok(v) => {
                let (packet, inner_length) = v;
                packet_length += inner_length;
                values.push(packet.value);
                packets.push(packet);
              }
              Err(e) => return Err(e),
            }
          }
        }
        _ => return Err("Invalid length type"),
      }
      return Ok((
        Packet {
          version: version,
          value: match p_type {
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => {
              if values[0] > values[1] {
                1
              } else {
                0
              }
            }
            6 => {
              if values[0] < values[1] {
                1
              } else {
                0
              }
            }
            7 => {
              if values[0] == values[1] {
                1
              } else {
                0
              }
            }
            _ => 0,
          },
          sub_packets: Some(packets),
        },
        packet_length,
      ));
    }
  }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Packet> {
  let mut bit_vec: Vec<char> = input
    .chars()
    .map(|c| format!("{:04b}", usize::from_str_radix(&c.to_string(), 16).unwrap()))
    .collect::<Vec<String>>()
    .join("")
    .chars()
    .collect();

  let mut packets = Vec::new();

  while bit_vec.contains(&'1') {
    let result = parse_next_packet(&mut bit_vec);
    match result {
      Ok(v) => {
        let (packet, _) = v;
        packets.push(packet);
      }
      Err(_e) => {}
    }
  }

  return packets;
}

fn sum_version(packet: &Packet) -> usize {
  let mut sum: usize = packet.version as usize;

  if let Some(sub_packets) = &packet.sub_packets {
    for sub_packet in sub_packets {
      sum += sum_version(&sub_packet);
    }
  }

  return sum;
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<Packet>) -> usize {
  let mut sum = 0;
  for packet in input {
    sum += sum_version(packet);
  }
  return sum;
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Vec<Packet>) -> u64 {
  return input[0].value;
}
