use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::fmt::Write;

#[allow(dead_code)]
pub fn print_hex(ls: &Vec<u8>) {
    for i in ls {
        print!("{:01$x} ", i, 2);
    }
    print!("\n");
}

#[allow(dead_code)]
pub fn print_ascii(ls: &Vec<u8>) {
    for i in ls {
        print!("{}", *i as char);
    }
    print!("\n");
}

#[allow(dead_code)]
pub fn to_hex_string(ls: &Vec<u8>) -> String {
    let mut output = String::new();
    for i in ls {
        write!(&mut output, "{:01$x} ", i, 2).expect("Couldn't write to string.");
    }
    return output;
}

#[allow(dead_code)]
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

pub fn to_utf8(ls: &Vec<u8>) -> String {
    match std::str::from_utf8(ls) {
        Ok(s) => s.to_string(),
        Err(_) => "".to_string(),
    }
}

pub fn decode_uleb128(bytes: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;
    let mut shift = 0;
    //println!("//////////// decode_uleb128 {}", to_hex_string(bytes));
    for byte in bytes {
        //println!("//////////// byte {:01$x}", byte, 2);
        result |= ((*byte & 0x7f) as u32) << shift;
        shift += 7;
        //println!("//////////// r:{} s:{}", result, shift);
        if 0 == *byte & 0x80 { break; }
    }

    //println!("//////////// result: {}", result);
    return result as u32;
}