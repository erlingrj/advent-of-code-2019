use std::fs;

fn mass_to_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn mass_to_fuel_2(mass: i32, acc: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        mass_to_fuel_2(fuel, acc+fuel)
    } else {
        acc
    }
}

fn day1() {
    let module_mass: Vec<i32> = fs::read_to_string("src/input.txt").unwrap().split('\n').flat_map(|line| line.parse::<i32>()).collect::<Vec<i32>>();
 
    println!("Initial fuel: {}",module_mass.iter().map(|mass| mass_to_fuel(*mass)).fold(0, |acc, x| acc + x));
    
    println!("More fuel: {}", module_mass.iter().map(|mass| mass_to_fuel_2(*mass, 0)).fold(0, |acc, x| acc + x));

}
