use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    let mut buf = Vec::new();
    File::open("README.md")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    let str = String::from_utf8(buf).unwrap();
    File::create("WEADME.md")
        .unwrap()
        .write_all(uwuifier::uwuify_str_sse(&str).as_bytes())
        .unwrap();
    println!("README.md copied to WEADME.md nyaaa~");
}
