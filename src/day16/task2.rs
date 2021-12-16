use crate::utils::load_input;

pub fn solve() -> usize {
    let input = load_input(16);
    let input = input.get(0).unwrap();
    let binary: String = input
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect();
    let (packet, _) = parse_packet(&binary);
    calculate_value(&packet)
}

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    sub_packets: Vec<Packet>,
}

fn parse_packet(data: &str) -> (Packet, usize) {
    let version = usize::from_str_radix(data.get(0..3).unwrap(), 2).unwrap();
    let type_id = usize::from_str_radix(data.get(3..6).unwrap(), 2).unwrap();
    let mut sub_packets = Vec::new();
    let mut already_parsed = 6;
    let mut value = 0;
    match type_id {
        4 => {
            let mut groups = Vec::new();
            loop {
                let group = data.get(already_parsed..already_parsed + 5).unwrap();
                groups.push(group.get(1..).unwrap());
                already_parsed += 5;
                if group.get(..1).unwrap() == "0" {
                    break;
                }
            }
            value = usize::from_str_radix(groups.join("").as_str(), 2).unwrap();
        }
        _ => {
            let length_type_id = data.get(6..7).unwrap();
            already_parsed += 1;
            match length_type_id {
                "0" => {
                    already_parsed += 15;
                    let total_bits_to_parse = usize::from_str_radix(data.get(7..22).unwrap(), 2)
                        .unwrap()
                        + already_parsed;
                    while already_parsed < total_bits_to_parse {
                        let (packet, parsed_bits) =
                            parse_packet(data.get(already_parsed..total_bits_to_parse).unwrap());
                        sub_packets.push(packet);
                        already_parsed += parsed_bits
                    }
                }
                "1" => {
                    already_parsed += 11;
                    let total_packets = usize::from_str_radix(data.get(7..18).unwrap(), 2).unwrap();
                    for _ in 0..total_packets {
                        let (packet, parsed_bits) =
                            parse_packet(data.get(already_parsed..).unwrap());
                        sub_packets.push(packet);
                        already_parsed += parsed_bits
                    }
                }
                _ => panic!(""),
            }
        }
    }
    (
        Packet {
            version,
            type_id,
            value,
            sub_packets,
        },
        already_parsed,
    )
}

fn calculate_value(packet: &Packet) -> usize {
    match packet.type_id {
        0 => packet
            .sub_packets
            .iter()
            .map(|packet| calculate_value(packet))
            .sum(),
        1 => packet
            .sub_packets
            .iter()
            .fold(1, |product, packet| product * calculate_value(packet)),
        2 => packet
            .sub_packets
            .iter()
            .map(|packet| calculate_value(packet))
            .min()
            .unwrap(),
        3 => packet
            .sub_packets
            .iter()
            .map(|packet| calculate_value(packet))
            .max()
            .unwrap(),
        4 => packet.value,
        5 => {
            (calculate_value(packet.sub_packets.get(0).unwrap())
                > calculate_value(packet.sub_packets.get(1).unwrap())) as usize
        }
        6 => {
            (calculate_value(packet.sub_packets.get(0).unwrap())
                < calculate_value(packet.sub_packets.get(1).unwrap())) as usize
        }
        7 => {
            (calculate_value(packet.sub_packets.get(0).unwrap())
                == calculate_value(packet.sub_packets.get(1).unwrap())) as usize
        }
        _ => panic!(),
    }
}
