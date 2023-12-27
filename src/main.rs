use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    // Entering raw mode
    let _stdout = stdout().into_raw_mode().unwrap();
    // Note - 1. Calling this on stdout because the terminals have their states controlled by the
    // the writer and not the reader.
    // Note - 2. We're assigning the result of .into_raw_mode() to _stdout, just to store the value
    // till the end (also by adding a '_' there, because it'll persist the value and not let it
    // delete till the program ends) (borrowchecker 😉)
    
    
    // Function to detect if ctrl is pressed along with a key
    fn to_ctrl_byte(c: char) -> u8 {
        let byte = c as u8;
        byte & 0b0001_1111
    }

    fn die(e: std::io::Error) {
        panic!("{}", e);
    }

    // Taking input and doing operations (in raw mode now)
    let input = io::stdin();
    for b in input.bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                }
                else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        };
    }
}
