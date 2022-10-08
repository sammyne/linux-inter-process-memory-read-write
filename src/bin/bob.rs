use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

fn main() {
    let args: Vec<String> = std::env::args().take(4).collect();
    if args.len() != 4 {
        println!("{} pid addr length", args[0]);
        std::process::exit(1);
    }

    let pid: u32 = args[1].parse().expect("parse pid");
    let addr = u64::from_str_radix(args[2].trim_start_matches("0x"), 16).expect("parse addr");
    let length: usize = args[3].parse().expect("parse length");

    let mem_path = format!("/proc/{pid}/mem");
    println!("opening {mem_path}, address is {addr:x}");
    let mut mem = File::options()
        .read(true)
        .write(true)
        .open(&mem_path)
        .expect(&format!("open '{mem_path}'"));

    let _ = mem
        .seek(SeekFrom::Start(addr))
        .expect(&format!("offset mem to {addr}"));

    let foo = {
        let mut buf = vec![0u8; length];
        let _ = mem.read_exact(buf.as_mut_slice()).expect("read buf");

        let out = String::from_utf8(buf).expect("parse buf as string");
        println!("string at 0x{addr:x} in process {pid} is:\n{}", out);
        out.replace("Alice", "Bob:)") // make the words to replace be the same length
    };

    let _ = mem
        .seek(SeekFrom::Start(addr))
        .expect(&format!("offset mem to {addr}"));
    {
        let buf = foo.as_bytes();
        let _ = mem.write_all(buf).expect("write back new string");
    }
}
