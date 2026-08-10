mod parser;
use parser::{Question, parser};
use std::{process::Command, io};

fn main() {
    Command::new("clear");
    println!("Welcome To QUIZZ - www.github.com/castlesp5");
    let mut vector: Vec<Question> = vec![];
    parser(&mut vector);
    // for elem in vector {
    //     println!("{} == {}", elem.question, elem.answer);
    // }
    let mut i = 0;
    while i < vector.len(){
        let mut buffer = String::new();
        println!("Question : {}", vector[i].question);
        io::stdin().read_line(&mut buffer).expect("FAILED TO READ BUFFER");
        if buffer.trim() == vector[i].answer.trim() {
            println!("CORRECT!!!!!! +10points");
            i += 1;
        }
        else {
            println!("INCORRECT!!! - correct answer is {}", vector[i].answer);
        }
    }
    // println!("SUCCESS");
}
