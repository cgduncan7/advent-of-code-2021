use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: u64,
    sub_packets: Vec<Packet>
}

fn create_bit_deque(hex_string: &str) -> VecDeque<bool> {
    let mut bits: VecDeque<bool> = VecDeque::new();
    for ch in hex_string.chars() {
        let val: u8 = ch.to_digit(16).unwrap().try_into().unwrap();
        let mut mask: u8 = 0b1000;
        while mask > 0 {
            bits.push_back(val & mask > 0);
            mask = mask >> 1;
        }
    }
    bits
}

fn take_bit(bits: &mut VecDeque<bool>) -> bool {
    bits.pop_front().unwrap()
}

fn take_bits(bits: &mut VecDeque<bool>, num: u32) -> VecDeque<bool> {
    let mut ret_bits = VecDeque::new();
    for _ in 0..num {
        ret_bits.push_back(take_bit(bits));
    }
    ret_bits
}

fn take(bits: &mut VecDeque<bool>, num: u32) -> u32 {
    let mut binary_string = String::new();
    for bit in take_bits(bits, num) {
        if bit {
            binary_string.push('1');
        } else {
            binary_string.push('0');
        }
    }
    u32::from_str_radix(&binary_string, 2).unwrap()
}

fn get_packet_version(bits: &mut VecDeque<bool>) -> u8 {
    take(bits, 3) as u8
}

fn get_packet_type_id(bits: &mut VecDeque<bool>) -> u8 {
    take(bits, 3) as u8
}

fn get_literal_value(bits: &mut VecDeque<bool>) -> u64 {
    let mut literal_value_bits: VecDeque<bool> = VecDeque::new();
    loop {
        let cnt = take_bit(bits);
        literal_value_bits.append(&mut take_bits(bits, 4));

        if !cnt {
            break;
        }
    }
    
    let mut binary_string = String::new();
    for bit in literal_value_bits {
        if bit {
            binary_string.push('1');
        } else {
            binary_string.push('0');
        }
    }
    u64::from_str_radix(&binary_string, 2).unwrap()
}

fn decode_sub_packets(bits: &mut VecDeque<bool>) -> Vec<Packet> {
    let mut packets = Vec::new();

    while bits.len() > 6 {
        packets.push(decode_packet(bits));
    }

    packets
}

fn decode_packet(bits: &mut VecDeque<bool>) -> Packet {
    let version = get_packet_version(bits);
    let type_id = get_packet_type_id(bits);

    let mut sub_packets = Vec::new();

    let value = match type_id {
        4 => get_literal_value(bits),
        0 => {
            if take_bit(bits) {
                let num_sub_packets = take(bits, 11);
                for _ in 0..num_sub_packets {
                    sub_packets.push(decode_packet(bits));
                }
            } else {
                let num_bits = take(bits, 15);
                let mut sub_packet_bits = take_bits(bits, num_bits);
                sub_packets = decode_sub_packets(&mut sub_packet_bits);
            };

            sub_packets.iter().fold(0, |acc, packet| acc + packet.value)
        },
        1 => {
            if take_bit(bits) {
                let num_sub_packets = take(bits, 11);
                for _ in 0..num_sub_packets {
                    sub_packets.push(decode_packet(bits));
                }
            } else {
                let num_bits = take(bits, 15);
                let mut sub_packet_bits = take_bits(bits, num_bits);
                sub_packets = decode_sub_packets(&mut sub_packet_bits);
            };

            sub_packets.iter().fold(1, |acc, packet| acc * packet.value)
        },
        2 => {
            if take_bit(bits) {
                let num_sub_packets = take(bits, 11);
                for _ in 0..num_sub_packets {
                    sub_packets.push(decode_packet(bits));
                }
            } else {
                let num_bits = take(bits, 15);
                let mut sub_packet_bits = take_bits(bits, num_bits);
                sub_packets = decode_sub_packets(&mut sub_packet_bits);
            };

            sub_packets.iter().fold(u64::MAX, |acc, packet| if packet.value < acc { packet.value } else { acc })
        },
        3 => {
            if take_bit(bits) {
                let num_sub_packets = take(bits, 11);
                for _ in 0..num_sub_packets {
                    sub_packets.push(decode_packet(bits));
                }
            } else {
                let num_bits = take(bits, 15);
                let mut sub_packet_bits = take_bits(bits, num_bits);
                sub_packets = decode_sub_packets(&mut sub_packet_bits);
            };

            sub_packets.iter().fold(u64::MIN, |acc, packet| if packet.value > acc { packet.value } else { acc })
        },
        5 => {
            if take_bit(bits) {
                let num_sub_packets = take(bits, 11);
                for _ in 0..num_sub_packets {
                    sub_packets.push(decode_packet(bits));
                }
            } else {
                let num_bits = take(bits, 15);
                let mut sub_packet_bits = take_bits(bits, num_bits);
                sub_packets = decode_sub_packets(&mut sub_packet_bits);
            };

            let first = sub_packets.get(0).unwrap();
            let second = sub_packets.get(1).unwrap();

            if first.value > second.value {
                1
            } else {
                0
            }
        },
        6 => {
            if take_bit(bits) {
                let num_sub_packets = take(bits, 11);
                for _ in 0..num_sub_packets {
                    sub_packets.push(decode_packet(bits));
                }
            } else {
                let num_bits = take(bits, 15);
                let mut sub_packet_bits = take_bits(bits, num_bits);
                sub_packets = decode_sub_packets(&mut sub_packet_bits);
            };

            let first = sub_packets.get(0).unwrap();
            let second = sub_packets.get(1).unwrap();

            if first.value < second.value {
                1
            } else {
                0
            }
        },
        7 => {
            if take_bit(bits) {
                let num_sub_packets = take(bits, 11);
                for _ in 0..num_sub_packets {
                    sub_packets.push(decode_packet(bits));
                }
            } else {
                let num_bits = take(bits, 15);
                let mut sub_packet_bits = take_bits(bits, num_bits);
                sub_packets = decode_sub_packets(&mut sub_packet_bits);
            };

            
            let first = sub_packets.get(0).unwrap();
            let second = sub_packets.get(1).unwrap();

            if first.value == second.value {
                1
            } else {
                0
            }
        },
        _ => panic!("Unsupported type id"),
    };

    Packet {
        version,
        type_id,
        value,
        sub_packets
    }
}

fn total_packet_versions(packet: &Packet) -> u64 {
    if packet.sub_packets.len() == 0 {
        return packet.version as u64;
    }

    packet.sub_packets.iter().fold(packet.version as u64, |acc, cur| acc + total_packet_versions(cur))
}

fn problem1(lines: &mut std::str::Lines) -> u64 {
    let input = lines.next().unwrap();

    let mut bit_deque = create_bit_deque(input);
    let packet = decode_packet(&mut bit_deque);

    total_packet_versions(&packet)
}

fn problem2(lines: &mut std::str::Lines) -> u64 {
    let input = lines.next().unwrap();

    let mut bit_deque = create_bit_deque(input);
    decode_packet(&mut bit_deque).value
}

fn main() {
    let mut file = match File::open("./data/input.txt") {
        Err(_) => panic!("Failed to open file"),
        Ok(f) => f,
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        panic!("Failed to read file");
    }

    let lines = contents.lines();
    println!("Problem1: {}", problem1(&mut lines.clone()));
    println!("Problem2: {}", problem2(&mut lines.clone()));
}
