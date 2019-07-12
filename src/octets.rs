use std::mem::size_of;

const SEGMENT_SIZE: usize = size_of::<usize>();

pub fn to_octets(value: usize) -> Vec<u8> {
  let mut octets: Vec<u8> = Vec::with_capacity(SEGMENT_SIZE);

  for part in 0..SEGMENT_SIZE {
    let mut bitmask: usize = 0;
    let base = 8 * part;
    for exp in base..(base + 8) {
      bitmask += 1 << exp;
    }

    let octet = (value & bitmask) >> base;
    octets.push(octet as u8);

    // println!("using bitmask={:b} with value={:b}, octet={:b}, shift={}", bitmask, value, octet, base);
  }

  octets
}

pub fn from_octets(octets: &[u8]) -> usize {
  let mut value: usize = 0;

  for part in 0..SEGMENT_SIZE {
    let part_val = octets[part];
    let part_val_shifted = (part_val as usize) << (part * SEGMENT_SIZE);
    value |= part_val_shifted;
  }

  value
}
