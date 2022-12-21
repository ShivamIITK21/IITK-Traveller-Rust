use text_io::read;
use std::process;

pub fn operate(location:i32, state:&mut crate::program_state::ProgramState){
    let tape = &mut state.tape;
    
    match location{
        0 => {},
        1 => process::exit(0),
        2 => {
            let i: i32 = read!();
            tape[state.mem1] = i;
        },
        3 => {
            let i: i32 = read!();
            tape[state.mem2] = i;
        }
        4 => {
            tape[state.mem3] = tape[state.mem1]+tape[state.mem2];
        }
        5 => {
            tape[state.mem3] = tape[state.mem1]*tape[state.mem2];
        }
        6 => {
            tape[state.mem3] = tape[state.mem1]-tape[state.mem2];
        }
        7 => {
            tape[state.mem3] = tape[state.mem1]/tape[state.mem2];
        }
        8 => {
            tape[state.mem1] = tape[state.mem3];
        }
        9 => {
            tape[state.mem3] = tape[state.mem1];
        }
        10 => {
            tape[state.mem2] = tape[state.mem3];
        }
        11 => {
            tape[state.mem3] = tape[state.mem2];
        }
        12 => {
            println!("{}", tape[state.mem1]);
        }
        13 => {
            println!("{}", tape[state.mem2]);
        }
        14 => {
            if tape[state.mem1] > tape[state.mem2] {
                state.location = 15;
            }
            else{
                state.location = 16;
            }
        }
        15 => {}
        16 => {}
        17 => {
            if tape[state.mem1] < tape[state.mem2]{
                state.location = 18;
            }
            else{
                state.location = 19;
            }
        }
        18 => {}
        19 => {}
        20 => {
            if tape[state.mem1] == tape[state.mem2] {
                state.location = 21;
            }
            else{
                state.location = 22;
            }
        }
        21 => {}
        22 => {}
        23 => {
            tape[state.mem1] += 1;
        }
        24 => {
            tape[state.mem2] += 1;
        }
        25 => {
            state.cond += 1;
        }
        26 => {
            tape[state.mem1] -= 1;
        }
        27 => {
            tape[state.mem2] -= 1;
        }
        28 => {
            state.cond -= 1;
        }
        29 => {
            tape[state.mem1] = 0;
        }
        30 => {
            tape[state.mem2] = 0;
        }
        31 => {
            tape[state.mem3] = 0;
        }
        32 => {
            state.cond = 0;
        }
        33 => {
            state.mem1 += 1;
        }
        34 => {
            state.mem2 += 1;
        }
        35 => {
            state.mem3 += 1;
        }
        36 => {
            state.mem1 -= 1;
        }
        37 => {
            state.mem2 -= 1;
        }
        38 =>{
            state.mem3 -= 1;
        }
        _ => {}
    }
}