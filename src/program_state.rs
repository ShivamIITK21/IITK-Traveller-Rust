pub struct ProgramState{
    pub mem1: i32,
    pub mem2: i32,
    pub mem3: i32,
    pub cond: i32,
    pub tape: Vec<i32>
}


impl ProgramState{
    pub fn new() -> ProgramState{
        let tape = vec![0; 4096];

        let state = ProgramState{
            mem1: 0,
            mem2: 0,
            mem3: 0,
            cond: 0,
            tape: tape
        };

        return state;
    }       
}