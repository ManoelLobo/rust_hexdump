use std::{fs::File, io::Read};

const BYTES_PER_LINE: usize = 16;

fn main() {
    let arg1 = std::env::args().nth(1);

    let filename = arg1.expect("usage xview <filename>");

    let mut file = File::open(filename).expect("file not found");
    let mut position = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while file.read_exact(&mut buffer).is_ok() {
        print!("[0x{:08x}] ", position);

        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }

        println!();
        position += BYTES_PER_LINE;
    }
}
