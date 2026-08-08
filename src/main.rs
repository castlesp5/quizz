use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;



fn parser(){
    let f = match File::open("./questions/quiz.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            exit(0);
        }
    };
    let reader = io::BufReader::new(f);
    for line in reader.lines(){
        let line = match line {
            Ok(line) => line,
            Err(_) => exit(0),
        };
        println!("{}", line);
    }
}



fn main() {
    parser();
    println!("SUCCESS");
}
