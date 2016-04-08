use std::env;
use std::str;

extern crate lexbuf;

use lexbuf::*;

fn main() {
    let r = env::args().nth(1).unwrap();
    let mut buf = ReadLexBuf::new(r.as_bytes());
    let mut c = buf.get();
    let mut i = 0;
    while c != 0 {
        print!("{}", c as char);
        if i % 3 == 0 {
            println!("\nValidated: {}", str::from_utf8(&buf.validate()).unwrap());
        } else if i % 4 == 0 {
            println!("\nGIVEUP!");
            buf.give_up();
        }
        c = buf.get();
        i += 1;
    }
}
