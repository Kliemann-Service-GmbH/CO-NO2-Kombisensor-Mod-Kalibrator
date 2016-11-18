// Test Programm um das Byteorder Crate zu testen.
extern crate byteorder;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

fn main() {
    let bytes = [ 0, 2, 9, 0, 10 , 2, 32];
    let mut buf = Cursor::new(&bytes[0..4]);
    let num = buf.read_u32::<BigEndian>().unwrap();

    println!("num: {:?}", num);
    println!("{}", format!("{}.{}.{}", bytes[0], bytes[1], bytes[2]))
}
