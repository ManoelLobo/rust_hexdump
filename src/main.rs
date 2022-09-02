use std::io::Read;

const BYTES_PER_LINE: usize = 16;
// r#""# is a raw string literal: double quotes are not escaped
// b flags to treat content as bytes, not UTF-8
const INPUT: &[u8] = br#"
fn main() {
    println!("Hello, world!");
}
"#;

fn main() -> std::io::Result<()> {
    let mut buffer = vec![];
    INPUT.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);

        for byte in line {
            print!("{:02x} ", byte);
        }

        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
