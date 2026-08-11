mod parser;
use parser::{Question, parser};
use std::{process::{Command, exit}, io, env};

fn clear_screen() {
    Command::new("clear").status().unwrap_or_else(|_| {
        eprintln!("failed to run command 'clear'.");
        exit(2);
    });
println!("    _______
   /      /,
  /      //
 /______//
(______(/
  ___  _   _ ___ __________      _ _
 / _ \\| | | |_ _|__  /__  /     | | |
| | | | | | || |  / /  / /      | | |
| |_| | |_| || | / /_ / /_ _ _ _|_|_|
 \\__\\_\\\\___/|___/____/____(_|_|_|_|_)
            github:@castlesp5");
}

fn main() {
    clear_screen();
    let mut vector: Vec<Question> = vec![];
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid number of arguments...");
        exit(0);
    }
    parser(&mut vector, &args[1]);
    let mut i = 0;
    let mut score = 0;
    while i < vector.len(){
        let mut buffer = String::new();
        println!("Question : {}", vector[i].question);
        io::stdin().read_line(&mut buffer).expect("FAILED TO READ BUFFER");
        if buffer.trim() == vector[i].answer.trim() {
            clear_screen();
            score += 10;
            println!("Current Score is : {}", score);
            println!("CORRECT!!!!!! +10points\n Press [ENTER] to continue....");
            i += 1;
            loop {
                let mut mini_buffer = String::new();
                io::stdin().read_line(&mut mini_buffer).expect("Failed..");
                println!("{mini_buffer}");
                if mini_buffer.trim().is_empty() {
                    clear_screen();
                    break;
                }
                else {
                    continue;
                }
            }
        }
        else {
            clear_screen();
            println!("INCORRECT!!! - correct answer is {}", vector[i].answer);
        }
    }
    println!("SUCCESS");
    println!("YOU HAVE COMPLETED ALL QUESTIONS WITH SCORE : {}", score);
}
