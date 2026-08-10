use std::fs::File;
use std::io::Read;
use std::process::exit;

pub struct Question {
    pub question: String,
    pub answer: String,
}


pub fn parser(vector: &mut Vec<Question>) -> &mut Vec<Question>{
    let mut reader = String::new();
    let _ = File::open("./questions/quiz.txt")
        .unwrap_or_else(|_| {
            eprintln!("File was not found");
            exit(2);
        })
    .read_to_string(&mut reader)
    .expect("Error while reading buffer");
    for line in reader.lines(){
        if line == "" || line.starts_with("#"){
            continue;
        }
        let parts: Vec<&str> = line.split(";").collect();
        if parts.len() < 2{
            eprintln!("ERROR in '{}', you must forgot ';'", line);
            exit(2);
        }
        let squestion = Question{
            question: parts[0].to_string(),
            answer: parts[1].to_string(),
        };
        vector.push(squestion);
    }
    vector
}
