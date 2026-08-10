mod parser;
use parser::{Question, parser};
use std::process::{exit, Command};

fn main() {
    let mut vector: Vec<Question> = vec![];
    parser(&mut vector);
    // for elem in vector {
    //     println!("{} == {}", elem.question, elem.answer);
    // }
    loop {
        Command::new("clear");
        exit(0);
    }
    println!("SUCCESS");
}
