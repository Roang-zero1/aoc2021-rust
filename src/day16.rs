#[derive(Debug)]
struct Packet {
  version: u8,
  type_id: u8,
  value: Option<u32>,
  sub_packets: Option<Vec<Packet>>,
}

fn drain_rest(bit_vec: &mut Vec<char>, len: usize) {
  bit_vec.drain(0..(4 - len % 4));
}

fn parse_next_packet(bit_vec: &mut Vec<char>) -> Option<Packet> {
  let mut len = 6;
  let version = u8::from_str_radix(&bit_vec.drain(0..3).collect::<String>(), 2).unwrap();
  let p_type = u8::from_str_radix(&bit_vec.drain(0..3).collect::<String>(), 2).unwrap();

  match p_type {
    4 => {
      let mut data_vec: Vec<char> = Vec::new();
      while {
        len += 5;
        let mut bits = bit_vec.drain(0..5);
        let next = bits.next().unwrap();
        data_vec.append(&mut bits.collect::<Vec<char>>());
        next == '1'
      } {}
      drain_rest(bit_vec, len);
      Some(Packet {
        version: version,
        type_id: p_type,
        value: Some(u32::from_str_radix(&data_vec.into_iter().collect::<String>(), 2).unwrap()),
        sub_packets: None,
      })
    }
    _ => None,
  }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<char> {
  let mut bit_vec: Vec<char> = input
    .chars()
    .map(|c| format!("{:04b}", usize::from_str_radix(&c.to_string(), 16).unwrap()))
    .collect::<Vec<String>>()
    .join("")
    .chars()
    .collect();

  println!("{:?}", bit_vec);

  let packet = parse_next_packet(&mut bit_vec);
  println!("{:?}", packet);

  return Vec::new();
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[char]) -> u32 {
  println!("{:?}", input);
  return 0;
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[char]) -> u32 {
  return 0;
}
