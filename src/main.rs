mod program_state;
mod parser;
mod landmarks;
mod graph;
mod operations;
mod interpreter;

fn main(){
    interpreter::interpret_from_file();
}