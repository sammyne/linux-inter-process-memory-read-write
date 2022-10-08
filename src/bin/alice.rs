use std::{io, process};

fn main() {
    println!("this is Alice");

    let foo = "This is some text from Alice";

    let cmd = std::env::args().next().unwrap().replace("alice", "bob");

    println!("Now execute");
    println!("  sudo {} {} {:p} {}", cmd, process::id(), foo, foo.len());

    println!("Press any key");
    {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("wait for continue signal");
    }

    println!("foo has been changed to '{foo}'");
}
