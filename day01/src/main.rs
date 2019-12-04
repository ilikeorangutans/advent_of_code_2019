use std::io;

fn main() {
    let stdin = io::stdin();

    let mut total = 0;
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("line");
        if buffer == "" {
            break;
        }

        let mass: i64 = buffer.trim().parse().expect("expected ingeter");
        let needed = rocket_equation(mass);
        total += needed;
    }

    println!("{}", total);
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
