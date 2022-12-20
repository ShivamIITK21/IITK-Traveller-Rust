mod program_state;
mod parser;
mod landmarks;

use std::env;
use std::process;

fn main(){
    // let mut state = program_state::ProgramState::new();
    let args:Vec<String> = env::args().collect();

    let filename = match parser::take_arguments(&args) {
        Ok(s) => s,
        Err(_e) => {
            println!("Missing file name");
            process::exit(1);
        }
    };

    let code = match parser::get_code(&filename) {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading {:?}", e);
            process::exit(1)
        }
    };

    let parsed_code = match parser::parse_code(&code){
        Ok(v) => v,
        Err(i) => {
            println!("Syntax Error on line {}", i);
            process::exit(1);
        }
    };

    let landmarks = landmarks::LANDMARKS.lock().unwrap();
    println!("{:?}", parsed_code);
    println!("{:?}", landmarks);
}