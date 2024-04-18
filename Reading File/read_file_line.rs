use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error as Err;

fn read_file_line_by_line() -> Result<(), Box<dyn Err>> {
    let file = match File::open("src/items"){
        Ok(file) => file,
        Err(e) => return Err(e.into())
    };

    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        match line_result{
            Ok(line) => println!("{}", line),
            Err(e) => return Err(e.into())
        };
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Err>> {
    read_file_line_by_line()
}
