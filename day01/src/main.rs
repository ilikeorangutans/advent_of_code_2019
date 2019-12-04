use std::io;

fn main() {
    let mut mass = 100756;

    let needed = rocket_equation(mass);

    println!("mass: {}, fuel: {}", mass, needed);
}

fn rocket_equation(mass: i64) -> i64 {
    let fuel = calculate_fuel_required(mass);
    if fuel <= 0 {
        return 0;
    }

    fuel + rocket_equation(fuel)
}

fn calculate_fuel_required(mass: i64) -> i64 {
    (mass / 3) - 2
}
