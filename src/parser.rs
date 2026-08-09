use std::fs::File;
use std::io::Read;
use std::process::exit;

pub struct Question {
    pub question: String,
    pub answer: String,
}


pub fn parser(vector: &mut Vec<Question>) -> &mut Vec<Question>{
    let mut f = match File::open("./questions/quiz.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            exit(0);
        }
    };
    let mut reader = String::new();
    if let Err(_) = f.read_to_string(&mut reader) {
        exit(0);
    }
    for line in reader.lines(){
        if line == "" || line.starts_with("#"){
            continue;
        }
        let parts: Vec<&str> = line.split(";").collect();
        if parts.len() < 2{
            eprintln!("ERROR in '{}'", line);
            exit(0);
        }
        let squestion = Question{
            question: parts[0].to_string(),
            answer: parts[1].to_string(),
        };
        vector.push(squestion);
    }
    vector
}
