use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::fmt::Write;

pub fn print_hex(ls: &Vec<u8>) {
    for i in ls {
        print!("{:01$x} ", i, 2);
    }
    print!("\n");
}

pub fn print_ascii(ls: &Vec<u8>) {
    for i in ls {
        print!("{}", *i as char);
    }
    print!("\n");
}

pub fn to_hex_string(ls: &Vec<u8>) -> String {
    let mut output = String::new();
    for i in ls {
        write!(&mut output, "{:01$x} ", i, 2).expect("Couldn't write to string.");
    }
    return output;
}

pub fn to_binary_string(ls: &Vec<u8>) -> String {
    let mut output = String::new();
    for i in ls {
        write!(&mut output, "{:01$b} ", i, 8).expect("Couldn't write to string.");
    }
    return output;
}

pub fn to_decimal(ls: &Vec<u8>) -> u32 {
    let mut rdr = Cursor::new(ls);
    rdr.read_u32::<LittleEndian>().unwrap()
}

pub fn to_decimal_short(ls: &Vec<u8>) -> u16 {
    let mut rdr = Cursor::new(ls);
    rdr.read_u16::<LittleEndian>().unwrap()
}

pub fn to_ascii(ls: &Vec<u8>) -> String {
    ls.iter().map(|c| *c as char).collect()
}

pub fn decode_uleb128(bytes: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;
    let mut shift = 0;

    for byte in bytes {
        result |= ((*byte & 0x7f) as u32) << shift;
        shift += 7;
        if 0 == *byte & 0x80 { break; }
    }

    return result as u32;
}