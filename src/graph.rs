use std::collections::HashMap;
use std::process;
use crate::operations::operate;
use crate::program_state::ProgramState;

pub fn generate_graph(parsed_code:Vec<Vec<String>>) -> HashMap<i32, HashMap<i32, i32>>{
    let landmarks = crate::landmarks::LANDMARKS.lock().unwrap();

    let mut graph:HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    for instruction in parsed_code.iter(){
        let landmark1 = match landmarks.get(&instruction[0] as &str){
            Some(s) => s,
            None => {
                println!("{} is not a valid landmark", &instruction[0]);
                process::exit(1);
            }
        };

        let landmark2 = match landmarks.get(&instruction[2] as &str){
            Some(s) => s,
            None => {
                println!("{} is not a valid landmark", &instruction[2]);
                process::exit(1);
            }
        };

        let cond:i32 = match instruction[1].parse(){
            Ok(i) => i,
            Err(_) => {
                println!("Condtion variable can only take integer values");
                process::exit(1);
            }
        };
        
        match graph.get_mut(landmark1) {
            Some(map) => {
                match (*map).get_mut(&cond){
                    Some(_i) => {
                        println!("Program stuck on {}", instruction[0]);
                        process::exit(1);
                    },
                    None => {
                        map.insert(cond, *landmark2);
                    }
                }
            },
            None => {
                let mut temp:HashMap<i32, i32> = HashMap::new();
                temp.insert(cond, *landmark2);
                graph.insert(*landmark1, temp);
            }
        }
    }

    graph
}

pub fn traverse(graph: &HashMap<i32, HashMap<i32, i32>>, state:&mut ProgramState){
    
    while state.location != 1 {
        operate(state.location, state);

        match graph.get(&state.location){
            Some(map) => {
                match map.get(&state.cond){
                    Some(loc) => {
                        state.location = *loc;
                    },
                    None => {
                        println!("Program got stuck at a location");
                        process::exit(1);
                    }
                }
            }
            None => {
                println!("Program got stuck at a location");
                process::exit(1);
            }
        }
    }

    operate(1, state);
}
