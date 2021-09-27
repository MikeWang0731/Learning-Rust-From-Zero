use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

fn main() {
    let f = File::open("hello.txt");
    // File::open -> Ok(File),Err(IOError)
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Error Creating"),
            }
            Other_error => panic!("Error Opening"),
        },
    };
}

fn read_user_file() -> Result<String, io::Error> {
    let mut f1 = File::open("hello.txt")?;
    // 这里的？代替了match的Ok和Err结构体
    let mut s1 = String::new();
    f1.read_to_string(&mut s1)?;
    Ok(s1);
}
