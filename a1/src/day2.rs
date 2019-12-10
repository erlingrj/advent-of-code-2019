pub enum ProgramError {InvalidOpCode, InvalidPC}


pub fn execute_intcode(mut intcode: Vec<i32>, PC: usize) -> Result<Vec<i32>, ProgramError> {
    // Use declarative programming to solve this mafakka
    if PC < 0 || PC >= intcode.len() {
        return Err(ProgramError::InvalidPC);
    }

    
    

    match intcode[PC] {
        1 => {
            let op1 = intcode[intcode[PC+1] as usize];
            let op2 = intcode[intcode[PC+2] as usize];
            let dst = intcode[PC+3] as usize;
            intcode[dst] = op1 + op2;
        }
        2 => {
            let op1 = intcode[intcode[PC+1] as usize];
            let op2 = intcode[intcode[PC+2] as usize];
            let dst = intcode[PC+3] as usize;
            intcode[dst] = op1 * op2;
        }
        99 => return Ok(intcode),
        _ => return Err(ProgramError::InvalidOpCode)
    }
    execute_intcode(intcode, PC+4)
}

pub fn space_sweep(mut mem: Vec<i32>, desired_output: i32) -> Result<(i32, i32), String>  {
    for x in  0..99 {
        for y in 0..99 {
            mem[1] = x;
            mem[2] = y;
            let final_state = execute_intcode(mem.clone(),0);
            match final_state {
                Ok(v) => {
                    if v[0] == desired_output {
                        return Ok((x,y));
                    }
                }
                Err(_) => (),
            }

        }
    }
    return Err(String::from("Couldnt solve problem"));
}