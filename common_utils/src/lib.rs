use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub fn read_file(file_path: &str)-> io::Result<String>{
    let file = File::open(file_path)?;
    let mut bur_reader = BufReader::new(file);
    let mut contents = String::new();
    bur_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_file("test");
        match result {
            Ok(st) => {
                assert_eq!(st, "test");
            }
            Err(_) => {}
        }

    }
}
