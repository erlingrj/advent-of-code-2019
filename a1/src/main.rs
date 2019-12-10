use std::fs;

mod lib;
mod day1;
mod day2;


fn day2_1() {
    let code = lib::read_integer_file("data/day2.input", ",");
    let final_state = day2::execute_intcode(code, 0);
    match final_state {
        Ok(v) => println!("{:?}", v),
        Err(_) => println!("error")
    }
}

fn day2_2() {
    let code = lib::read_integer_file("data/day2.input", ",");
    let solution = day2::space_sweep(code, 19690720);
    match solution {
        Ok(s) => println!("{:?}",s),
        Err(e) => println!("{:?}",e)
    }
}

fn day2_test() {
    let code = lib::read_integer_file("data/day2.test", ",");
    let final_state = day2::execute_intcode(code, 0);
    match final_state {
        Ok(v) => println!("{:?}", v),
        Err(_) => println!("error")

    }
}

fn main() {
    day2_2();
}
