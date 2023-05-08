use crate::table::Table;

mod table;

const PRINTABLE: [bool; 256] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, true, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];

fn main() {
    let file = std::fs::read(std::env::args().collect::<Vec<_>>().get(1).unwrap()).unwrap();

    let mut table = Table::new(vec![
        "base".into(),
        "00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f".into(),
        "ascii".into(),
    ]);
    let chunks = file.chunks_exact(16);

    let mut last_pos = 0;

    for (i, bytes) in chunks.clone().enumerate() {
        let mut display = String::new();
        bytes.iter().for_each(|&b| {
            if PRINTABLE[b as usize] {
                display.push(char::from_u32(b as u32).unwrap())
            } else {
                display.push('·')
            }
        });

        table.insert(Vec::from([
            format!("{:04X}", i * 16),
            format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]),
            display
            ])
        );

        last_pos = (i + 1) * 16;
    }

    let remainder = chunks.remainder();

    if remainder.len() > 0 {
        let mut hex = Vec::new();
        let mut display = String::new();

        remainder.iter().for_each(|&b| {
            hex.push(format!("{:02X}", b));
            if PRINTABLE[b as usize] {
                display.push(char::from_u32(b as u32).unwrap())
            } else {
                display.push('·')
            }
        });

        table.insert(vec![
            format!("{:04X}", last_pos),
            format!("{:<47}", hex.join(" ")),
            format!("{:<16}", display),
        ]);
    }

    table.display()
}
