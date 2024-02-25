use std::io::{ErrorKind, Read};

fn main() {

    let mut stdin = std::io::stdin();
    
    let mut buf = [0; 1024];

    loop {
        match stdin.read(&mut buf) {
            Ok(0) => {
                break
            },
            Ok(n) => {
                match std::str::from_utf8(&buf[..n]) {
                    Ok(s) => println!("read '{}'", s),
                    Err(_) => println!("invalid UTF8: {:?}", &buf[..n])
                }
            },
            Err(err) => match err.kind() {
                ErrorKind::Interrupted => continue,
                _ => {
                    println!("error reading stdin: {}", err);
                    break
                }
            }
        }
    }
}
