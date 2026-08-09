mod parser;
use parser::Question;
use parser::parser;

fn main() {
    let mut vector: Vec<Question> = vec![];
    parser(&mut vector);
    for elem in vector {
        println!("{} == {}", elem.question, elem.answer);
    }
    println!("SUCCESS");
}
