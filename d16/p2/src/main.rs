use std::fs;

#[derive(Debug)]
enum ParseResult {
    OperatorResult(Operator),
    LiteralResult(Literal)
}

#[derive(Debug)]
struct Literal {
    version: u8,
    value: usize
}

#[derive(Debug)]
struct Operator {
    version: u8,
    operator_type: u8,
    parameters: Vec<ParseResult>
}

fn get_quintuplet(bytes: &[u8], start_bit: usize) -> u8 {
    let start_byte = start_bit / 8;
    let bits_taken = 8 - (start_bit % 8) as u8; // bits taken from the first byte
    let bits_remaining = 8 - bits_taken; // bits that could not be taken from the first byte
    let high_bits = (bytes[start_byte] & (1u16.checked_shl(bits_taken as u32).unwrap_or(256) - 1) as u8) << bits_remaining;
    let low_bits = match bits_taken {
        5 | 6 | 7 | 8 => 0,
        _ =>bytes[start_byte + 1].checked_shr(bits_taken as u32).unwrap_or(0)
    };

    let result = high_bits | low_bits;

    result
}

fn get_byte(bytes: &[u8], start_bit: usize) -> u8 {
    let start_byte = start_bit / 8;
    let bit_offset = start_bit % 8;
    let bits_taken = 8 - (start_bit % 8) as u8; // bits taken from the first byte
    let bits_remaining = 8 - bits_taken; // bits that could not be taken from the first byte
    let high_bits = (bytes[start_byte] & (1u16.checked_shl(bits_taken as u32).unwrap_or(256) - 1) as u8) << bits_remaining;
    let low_bits = match bits_taken {
        8 => 0,
        _ =>bytes[start_byte + 1].checked_shr(bits_taken as u32).unwrap_or(0)
    };

    let result = high_bits | low_bits;

    result
}

fn parse(bytes: &[u8], start: usize) -> (ParseResult, usize) {
    let first_byte = get_byte(bytes, start);
    let version_number = first_byte >> 5;
    let packet_type = (first_byte >> 2) & 0b111;
    match packet_type {
        4 => {
            let mut literal: usize = 0;
            let mut group_start_index = start + 6;
            loop {
                let group_in = get_quintuplet(bytes, group_start_index) >> 3;
                group_start_index += 5;
                literal <<=4;
                literal += (group_in & 0b1111) as usize;
                if group_in >> 4 == 0 {
                    break;
                }
            }

            (ParseResult::LiteralResult(Literal{
                version: version_number,
                value: literal
            }), group_start_index - start)
        }
        _ => {
            let mut sub_packets = Vec::new();
            let length_type = (first_byte >> 1) & 0b1;
            let total_packet_length = match length_type {
                1 => {
                    let sub_packets_high = (get_byte(bytes, start + 7) as u16) << 3; // 8 from here
                    let sub_packets_low = (get_byte(bytes, start + 15) >> 5) as u16; // 3 from here

                    let num_subpackets = sub_packets_high | sub_packets_low;
                    let mut subpacket_start = start + 18;
                    for _subpacket_num in 0.. num_subpackets {
                        let new_subpacket = parse(bytes, subpacket_start);
                        subpacket_start += new_subpacket.1;
                        sub_packets.push(new_subpacket.0);
                    }
                    subpacket_start - start
                },
                0 => {
                    let sub_packets_high = (get_byte(bytes, start + 7) as u16) << 7; // 8 from here
                    let sub_packets_low = (get_byte(bytes, start + 15) >> 1) as u16; // 7 from here

                    let subpackets_length = (sub_packets_high | sub_packets_low) as usize;

                    let mut subpacket_bits_read = 0usize;

                    let subpackets_head = start + 7 + 15;

                    while subpacket_bits_read < subpackets_length {
                        let new_subpacket = parse(bytes, subpackets_head + subpacket_bits_read);
                        subpacket_bits_read += new_subpacket.1;
                        sub_packets.push(new_subpacket.0);
                    }

                    subpackets_head + subpacket_bits_read - start
                },
                _ => panic!("Invalid length type")
            };

            (ParseResult::OperatorResult(
                Operator{
                    version: version_number,
                    operator_type: packet_type,
                    parameters: sub_packets
                }
            ), total_packet_length)
        }
    }
}

fn evaluate_tree(root: &ParseResult) -> usize {
    match root {
        ParseResult::LiteralResult(lit) => {
            lit.value  
        },
        ParseResult::OperatorResult(op) => {
            match op.operator_type {
                0 => {
                    op.parameters.iter().map(|x| evaluate_tree(x)).sum()
                },
                1 => {
                    op.parameters.iter().map(|x| evaluate_tree(x)).product()
                },
                2 => {
                    op.parameters.iter().map(|x| evaluate_tree(x)).min().expect("Invalid equation")
                },
                3 => {
                    op.parameters.iter().map(|x| evaluate_tree(x)).max().expect("Invalid equation")
                },
                5 => {
                    if evaluate_tree(&op.parameters[0]) > evaluate_tree(&op.parameters[1]) {1} else{0}
                },
                6 => {
                    if evaluate_tree(&op.parameters[0]) < evaluate_tree(&op.parameters[1]) {1} else{0}
                },
                7 => {
                    if evaluate_tree(&op.parameters[0]) == evaluate_tree(&op.parameters[1]) {1} else{0}
                },
                _ => panic!("Invalid op code")
            }
        }
    }
}

fn main() {
    let input_packet_raw = fs::read_to_string("input").expect("File not found").chars().map(|x| x.to_digit(16).expect(format!("Invalid hexadecimal {}", x).as_str()) as u8).collect::<Vec<u8>>();
    let mut input_packet = Vec::new();

    for packet_index in (0..input_packet_raw.len()).step_by(2) {
        let high_bits = input_packet_raw[packet_index] << 4;
        let low_bits = input_packet_raw[packet_index+1];
        let byte = high_bits | low_bits;
        input_packet.push(byte);
    }
    println!("{:?}", input_packet);
    let result = parse(&input_packet, 0);

    println!("{:?}", result.0);

    let tree_result = evaluate_tree(&result.0);

    println!("{}", tree_result);
}
